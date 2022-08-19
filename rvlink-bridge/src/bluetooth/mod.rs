use atomic::{Atomic, Ordering};
use bluer::gatt::remote::Characteristic;
use bluer::{Adapter, AdapterEvent, Device, Uuid};
use futures::{pin_mut, StreamExt};
use crossbeam_queue::SegQueue;
use rvlink_common::error::*;
use rvlink_proto::encoding::COBS;
use std::num::Wrapping;
use std::sync::Arc;
use tokio::select;
use tokio::sync::{Notify, RwLock};
use tokio::time::{sleep, Duration};

#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub enum BluetoothManagerState {
    #[default]
    Stopped,
    Scanning,
    Connecting,
    Handshaking,
    Running,
}

#[derive(Debug, Deref, Clone)]
pub struct BluetoothManager(Arc<BluetoothManagerInner>);

#[derive(Debug)]
pub struct BluetoothManagerInner {
    // session: Session,
    adapter: Adapter,
    device: RwLock<Option<Device>>,
    state: Atomic<BluetoothManagerState>,
    device_name: String,
    rx_queue: SegQueue<Vec<u8>>,
    rx_notify: Notify,
    tx_queue: SegQueue<Vec<u8>>,
    tx_notify: Notify,
}

#[allow(dead_code)]
impl BluetoothManager {
    /// This is the UUID for the primary bluetooth service
    const RVLINK_SERVICE: [u8; 16] = hex!("00000041 0200 a58e e411 afe28044e62c");

    // Services provided
    const KEX_SERVICE: [u8; 16] = hex!("00000010 0200 a58e e411 afe28044e62c"); // Key exchange
    const UNKNOWN: [u8; 16] = hex!("00000020 0200 a58e e411 afe28044e62c");
    const CAN_SERVICE: [u8; 16] = hex!("00000030 0200 a58e e411 afe28044e62c");
    const SERVICE_DEVICE_INFO: [u8; 16] = hex!("0000180a 0000 1000 8000 00805f9b34fb");
    const SERVICE_GENERIC_ATTRIBUTE: [u8; 16] = hex!("00001801 0000 1000 8000 00805f9b34fb");

    // Characteristics provided
    const SEED_CHAR: [u8; 16] = hex!("00000012 0200 a58e e411 afe28044e62c");
    const KEY_CHAR: [u8; 16] = hex!("00000013 0200 a58e e411 afe28044e62c");
    const CAN_VERSION: [u8; 16] = hex!("00000031 0200 a58e e411 afe28044e62c");
    const CAN_WRITE: [u8; 16] = hex!("00000033 0200 a58e e411 afe28044e62c");
    const CAN_READ: [u8; 16] = hex!("00000034 0200 a58e e411 afe28044e62c");

    // Constants used in the "key exchange"
    const RVLINK_KEY_SEED_CODE: u32 = 612643285;
    const RVLINK_UNLOCKED_RSP: [u8; 8] = hex!("556e6c6f636b6564");

    /// Creates a new instance of the bluetooth manager
    pub async fn new(device_name: String) -> Result<Self> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        let device = Default::default();
        let state = Default::default();
        adapter.set_powered(true).await?;
        Ok(Self(Arc::new(BluetoothManagerInner {
            rx_queue: SegQueue::new(),
            rx_notify: Default::default(),
            tx_queue: SegQueue::new(),
            tx_notify: Default::default(),
            adapter,
            device,
            state,
            device_name,
        })))
    }

    async fn get_device(&self) -> Result<Device> {
        Ok(self
            .device
            .read()
            .await
            .as_ref()
            .ok_or_else(|| AppError::Generic("Device not available!".into()))?
            .clone())
    }

    pub async fn recv(&self) -> Result<Vec<u8>> {
        let mut res = self.rx_queue.pop();
        while res.is_none() {
            self.rx_notify.notified().await;
            res = self.rx_queue.pop();
        }
        Ok(res.unwrap())
        // Ok(self.rx_recv.write().await.recv().await.unwrap_or_default())
    }

    pub async fn send(&self, data: Vec<u8>) -> Result<()> {
        self.tx_queue.push(data);
        self.tx_notify.notify_one();
        Ok(())
    }

    pub fn get_state(&self) -> BluetoothManagerState {
        self.state.load(Ordering::Relaxed)
    }

    fn set_state(&self, state: BluetoothManagerState) {
        self.state.store(state, Ordering::Relaxed);
    }

    async fn find_characteristic(
        &self,
        service_uuid: Uuid,
        char_uuid: Uuid,
    ) -> Result<Characteristic> {
        let device = self.get_device().await?;
        for service in device.services().await? {
            let uuid = service.uuid().await?;
            if uuid == service_uuid {
                debug!("Found service with ID {}", service_uuid);
                for char in service.characteristics().await? {
                    let uuid = char.uuid().await?;
                    if uuid == char_uuid {
                        debug!("Found characteristic with  UUID: {}", &uuid);
                        let char_flags = char.flags().await?;
                        debug!("Characteristic flags: {:?}", char_flags);
                        return Ok(char);
                    }
                }
            }
        }
        Err(AppError::Generic("Could not find characteristic!".into()))
    }

    pub async fn start(&self) -> Result<()> {
        let zelf = self.clone();
        tokio::task::spawn(async move {
            let zelf = zelf;
            loop {
                match zelf.get_state() {
                    BluetoothManagerState::Stopped => {
                        info!("Starting bluetooth scan loop...");
                        zelf.set_state(BluetoothManagerState::Scanning);
                    }
                    BluetoothManagerState::Scanning => match zelf.do_scan().await {
                        Ok(_) => {
                            zelf.set_state(BluetoothManagerState::Connecting);
                        }
                        Err(e) => {
                            warn!("Error occurred while scanning for devices! {:?}", e);
                            continue;
                        }
                    },
                    BluetoothManagerState::Connecting => match zelf.do_connect().await {
                        Ok(_) => {
                            zelf.set_state(BluetoothManagerState::Handshaking);
                        }
                        Err(e) => {
                            warn!("Error occurred while connecting! {:?}", e);
                            sleep(Duration::from_millis(750)).await;
                            info!("Retrying...");
                            continue;
                        }
                    },
                    BluetoothManagerState::Handshaking => match zelf.do_handshake().await {
                        Ok(true) => {
                            zelf.set_state(BluetoothManagerState::Running);
                        }
                        Ok(false) => {}
                        Err(e) => {
                            warn!("Error occurred while handshaking! {:?}", e);
                            zelf.set_state(BluetoothManagerState::Connecting);
                            sleep(Duration::from_millis(1500)).await;
                            continue;
                        }
                    },
                    BluetoothManagerState::Running => match zelf.do_run().await {
                        Err(e) => {
                            warn!("Error occurred in bluetooth main loop! Restarting connection process. {:?}", e);
                            zelf.set_state(BluetoothManagerState::Connecting);
                            continue;
                        }
                        _ => {}
                    },
                }
            }
        });
        Ok(())
    }

    fn unlock_seed(seed: u32) -> u32 {
        // Rotating numeral, increments every loop
        let mut rot: Wrapping<u32> = Wrapping(2654435769);
        // Unique value for the RVLink BLE gateway
        let mut code: Wrapping<u32> = Wrapping(Self::RVLINK_KEY_SEED_CODE);
        // Value retrieved from the seed service
        let mut seed: Wrapping<u32> = Wrapping(seed);

        for _ in 0..32 {
            seed += ((code << 4) + Wrapping(1131376761))
                ^ (code + rot)
                ^ ((code >> 5) + Wrapping(1919510376));
            code += ((seed << 4) + Wrapping(1948272964))
                ^ (seed + rot)
                ^ ((seed >> 5) + Wrapping(1400073827));
            rot += Wrapping(2654435769);
        }

        return seed.0;
    }

    /// Scan for the selected device and make it active
    async fn do_scan(&self) -> Result<()> {
        debug!(
            "Discovering devices using Bluetooth adapter {}",
            self.adapter.name()
        );

        let device_events = self.adapter.discover_devices().await?;
        pin_mut!(device_events);

        loop {
            if let Some(device_event) = device_events.next().await {
                match device_event {
                    AdapterEvent::DeviceAdded(addr) => {
                        let device = self.adapter.device(addr)?;
                        if self.device_name == device.name().await?.unwrap_or_default() {
                            if !device.is_trusted().await? {
                                device.set_trusted(true).await?;
                            }
                            let props = device.all_properties().await?;
                            for prop in props {
                                debug!("    {:?}", &prop);
                            }
                            *self.device.write().await = Some(device);
                            return Ok(());
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    /// Scan for the selected device and make it active
    async fn do_connect(&self) -> Result<()> {
        let device = self.get_device().await?;

        if !device.is_connected().await? {
            info!("Attempting to connect to device!");
            device.connect().await?;
        }
        info!("Device is connected!");

        for service in device.services().await? {
            let uuid = service.uuid().await?;
            debug!("Found Service UUID: {}", &uuid);
        }

        if !device.is_paired().await? {
            info!("Attempting to pair with device!");
            device.pair().await?;
        }
        info!("Device is paired!");
        Ok(())
    }

    async fn do_handshake(&self) -> Result<bool> {
        let kex_service_uuid = Uuid::from_slice(&Self::KEX_SERVICE).unwrap();
        let seed_char_uuid = Uuid::from_slice(&Self::SEED_CHAR).unwrap();
        let key_char_uuid = Uuid::from_slice(&Self::KEY_CHAR).unwrap();
        let seed_char = self
            .find_characteristic(kex_service_uuid, seed_char_uuid)
            .await?;
        let key_char = self
            .find_characteristic(kex_service_uuid, key_char_uuid)
            .await?;
        info!("Reading from seed service...");
        let in_data = seed_char.read().await?;
        if in_data == Self::RVLINK_UNLOCKED_RSP {
            info!("Device unlocked!");
            return Ok(true);
        }
        info!("Input data: {:?}", in_data);
        if in_data.len() != 4 {
            return Err(AppError::Generic(
                "Unexpected data length from key service!".into(),
            ));
        }
        let seed_u32: u32 = <u32>::from_be_bytes(in_data[0..4].try_into()?);
        let key_u32: u32 = Self::unlock_seed(seed_u32);
        info!("Writing to key service...");
        key_char.write(&key_u32.to_be_bytes()).await?;
        info!("waiting to allow device to unlock...");
        sleep(Duration::from_secs(1)).await;
        Ok(false)
    }

    async fn do_run(&self) -> Result<()> {
        let service_uuid = Uuid::from_slice(&Self::CAN_SERVICE).unwrap();
        let can_write_uuid = Uuid::from_slice(&Self::CAN_WRITE).unwrap();
        let can_read_uuid = Uuid::from_slice(&Self::CAN_READ).unwrap();
        let write_char = self
            .find_characteristic(service_uuid, can_write_uuid)
            .await?;
        let read_char = self
            .find_characteristic(service_uuid, can_read_uuid)
            .await?;
        let rx_recv = read_char.notify().await?;
        pin_mut!(rx_recv);
        loop {
            select! {
                _ = self.tx_notify.notified() => {
                    while let Some(tx_data) = self.tx_queue.pop() {
                        info!("Sending {:?}", tx_data);
                        let tx_data = COBS::encode(&tx_data)?;
                        write_char.write(&tx_data).await?;
                    }
                }
                Some(rx_data) = rx_recv.next() => {
                    let rx_data = COBS::decode(&rx_data)?;
                    self.rx_queue.push(rx_data);
                    self.rx_notify.notify_one();
                }
                _ = sleep(Duration::from_secs(30)) => {
                    warn!("No data for 30 seconds, is something wrong? Raising an error.");
                    return Err(AppError::Generic("No data received for 30 seconds!".into()));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BluetoothManager;

    #[test]
    /// Validates that the unlock process works as expected
    fn validate_kex_unlock() {
        let in_out_seeds = &[(0x54d7064au32, 0xb68a3bb3u32), (0xd22f4935, 0x42d8d17a)];
        for (i, (in_seed, out_seed)) in in_out_seeds.iter().enumerate() {
            let result = BluetoothManager::unlock_seed(*in_seed);
            assert_eq!(result, *out_seed);
            println!("Check {} passed", i);
        }
    }
}
