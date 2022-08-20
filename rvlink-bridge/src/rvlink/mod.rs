use crate::bluetooth::BluetoothManager;
use crate::devices::DeviceEntity;
use crate::devices::SystemEntityType;
use crate::mqtt::MqttManager;
use atomic::Atomic;
use fixed::{types::extra::U8, FixedU16};
use lockfree::map::Map;
use rand::Rng;
use rvlink_common::*;
use rvlink_proto::{events, *};
use std::sync::atomic::*;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, RwLock};
use tokio::time::{interval, sleep, Duration};

#[derive(Debug, Deref, Clone)]
pub struct RVLink(Arc<RVLinkInner>);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum DeviceState {
    #[default]
    Unknown,
    Switch(OnOff),
    Percentage(u8),
    Voltage(FixedU16<U8>),
}

impl DeviceState {
    pub fn state_string(&self) -> String {
        match self {
            DeviceState::Unknown => "unknown".into(),
            DeviceState::Switch(onoff) => onoff.to_string(),
            DeviceState::Percentage(pc) => format!("{}%", pc),
            DeviceState::Voltage(v) => format!("{}V", v),
        }
    }
}

#[derive(Debug, Default)]
pub struct DeviceEntry {
    pub entity: DeviceEntity,
    pub state: Atomic<DeviceState>,
    pub last_published: AtomicU64,
    pub last_published_state: Atomic<DeviceState>,
}

#[derive(Debug, Default)]
struct DeviceTable {
    pub devices: Arc<Map<u8, Arc<DeviceEntry>>>,
    pub device_count: AtomicU8,
    pub crc: AtomicU32,
    pub metadata_crc: AtomicU32,
}

#[derive(Debug)]
pub struct RVLinkInner {
    bluetooth: BluetoothManager,
    mqtt: RwLock<Option<MqttManager>>,
    msgnum: AtomicU16,
    cmdmap: Map<u16, mpsc::UnboundedSender<CommandResponse>>,
    device_tables: Map<u8, Arc<DeviceTable>>,
    device_id_lookup: Map<String, Arc<DeviceEntry>>,
    battery: Arc<DeviceEntry>,
}

#[allow(dead_code)]
impl RVLink {
    /// Create a new RVLink manager instance
    pub async fn new(bluetooth: BluetoothManager) -> Result<Self> {
        let mut rng = rand::thread_rng();
        let msgnum = AtomicU16::new(rng.gen());
        Ok(Self(Arc::new(RVLinkInner {
            bluetooth,
            msgnum,
            cmdmap: Default::default(),
            device_tables: Default::default(),
            mqtt: Default::default(),
            device_id_lookup: Default::default(),
            battery: Arc::new(DeviceEntry {
                entity: DeviceEntity::new_system(SystemEntityType::Battery).await,
                state: Default::default(),
                last_published: Default::default(),
                last_published_state: Default::default(),
            }),
        })))
    }

    async fn lookup_device(&self, uniq_id: &str) -> Option<Arc<DeviceEntry>> {
        self.device_id_lookup.get(uniq_id).map(|v| v.val().clone())
    }

    async fn publish_device_state(&self, entry: &DeviceEntry) {
        let cur_state = entry.state.load(Ordering::Relaxed);
        let last_state = entry.last_published_state.load(Ordering::Relaxed);
        let start = SystemTime::now();
        let last_timestamp = entry.last_published.load(Ordering::Relaxed);
        let cur_timestamp = start
            .duration_since(UNIX_EPOCH)
            .expect("You are a time traveler")
            .as_secs();

        // Avoid republishing unless it's been at least 20 seconds OR the state has changed
        if (cur_state != last_state) || ((cur_timestamp - last_timestamp) > 20) {
            entry
                .last_published_state
                .store(cur_state, Ordering::Relaxed);
            entry.last_published.store(cur_timestamp, Ordering::Relaxed);
            let zelf = self.clone();
            let device = entry.entity.clone();
            let state_str = cur_state.state_string();
            // Spawn a lightweight task so we don't block this one if MQTT is blocked
            tokio::task::spawn(async move {
                match zelf
                    .get_mqtt()
                    .await
                    .publish_device_state(&device, &state_str)
                    .await
                {
                    Ok(_) => {}
                    Err(e) => warn!("Could not update device state due to error! {:?}", e),
                }
            });
        }
    }

    async fn publish_device_info(&self, device: DeviceEntity) {
        let zelf = self.clone();
        tokio::task::spawn(async move {
            match zelf.get_mqtt().await.publish_device_info(&device).await {
                Ok(_) => {}
                Err(e) => warn!("Could not update device state due to error! {:?}", e),
            }
        });
    }

    pub async fn run_command(&self, uniq_id: &str, command: &str) -> Result<()> {
        if let Some(device) = self.lookup_device(uniq_id).await {
            let device = device.clone();
            if let Some((device_table_id, device_id)) = device.entity.get_device_address().await {
                warn!(
                    "Processing command {} to {}:{}",
                    command, device_table_id, device_id
                );
                match command {
                    "on" => {
                        self.send(ActionSwitch {
                            client_command_id: Default::default(),
                            device_table_id,
                            device_state: OnOff::On,
                            first_device_id: device_id,
                        })
                        .await?;
                    }
                    "off" => {
                        self.send(ActionSwitch {
                            client_command_id: Default::default(),
                            device_table_id,
                            device_state: OnOff::Off,
                            first_device_id: device_id,
                        })
                        .await?;
                    }
                    "open" => {
                        self.send(ActionMovement {
                            client_command_id: Default::default(),
                            device_table_id,
                            device_id: device_id,
                            device_state: RelayDirection::Open,
                        })
                        .await?;
                    }
                    "close" => {
                        self.send(ActionMovement {
                            client_command_id: Default::default(),
                            device_table_id,
                            device_id: device_id,
                            device_state: RelayDirection::Close,
                        })
                        .await?;
                    }
                    "stop" => {
                        self.send(ActionMovement {
                            client_command_id: Default::default(),
                            device_table_id,
                            device_id: device_id,
                            device_state: RelayDirection::Stop,
                        })
                        .await?;
                    }
                    cmd => warn!("Unrecognized command: {}", cmd),
                }
            }
        } else {
            warn!("Attempt to send command to unknown device id: {}", uniq_id);
        }
        Ok(())
    }

    pub async fn set_mqtt_manager(&self, mqtt: MqttManager) {
        *self.mqtt.write().await = Some(mqtt);
    }

    async fn get_mqtt(&self) -> MqttManager {
        self.mqtt.read().await.clone().unwrap()
    }

    /// Start the main loop to process incoming commands from the device
    pub async fn start(&self) -> Result<()> {
        tokio::task::spawn(self.clone().run_loop());
        tokio::task::spawn(self.clone().run_timers());
        Ok(())
    }

    async fn get_device_tables(&self) -> Result<Vec<Arc<DeviceTable>>> {
        let mut result = vec![];
        for t in self.device_tables.iter() {
            result.push(t.val().clone())
        }
        Ok(result)
    }

    pub async fn get_devices(&self) -> Result<Vec<Arc<DeviceEntry>>> {
        let mut result = vec![];
        if self.has_battery().await {
            result.push(self.battery.clone())
        }
        for table in self.get_device_tables().await? {
            for device in table.devices.iter() {
                result.push(device.val().clone());
            }
        }
        Ok(result)
    }

    /// Send a command to get device metadata from the specified device table
    async fn sync_devices_metadata(self, device_table_id: u8) -> Result<()> {
        let mut cmd = GetDevicesMetadata::default();
        cmd.device_table_id = device_table_id;
        cmd.start_device_id = 0;
        cmd.max_device_request_count = 255;
        let mut devices = vec![];
        let mut device_count = 255;
        let mut device_crc = 0;

        while device_count as usize != devices.len() {
            if devices.len() > 0 {
                warn!("Received an invalid metadata response! Ignoring and retrying...");
                let stutter = rand::thread_rng().gen_range(800..1500);
                sleep(Duration::from_millis(stutter)).await;
                devices.clear();
            };
            let responses = self.send(cmd.clone()).await?;
            for response in responses {
                match response {
                    GetDevicesMetadataResponse::Success(data) => {
                        for device in data.devices.into_iter() {
                            devices.push(device);
                        }
                    }
                    GetDevicesMetadataResponse::SuccessComplete(data) => {
                        device_count = data.device_count;
                        device_crc = data.device_metadata_table_crc;
                    }
                    GetDevicesMetadataResponse::Failure(_)
                    | GetDevicesMetadataResponse::FailureComplete(_) => {
                        warn!("Failure while trying to retrieve device metadata!");
                        device_count = 255;
                        devices.clear();
                        let stutter = rand::thread_rng().gen_range(800..1500);
                        sleep(Duration::from_millis(stutter)).await;
                    }
                }
            }
        }

        let device_table = self.get_or_add_device_table(device_table_id).await;
        for (i, device) in devices.into_iter().enumerate() {
            debug!("Found device metadata: {:?}", device);
            let (_, device_entry) = self.get_or_add_device(device_table_id, i as u8).await;
            device_entry
                .entity
                .update_from_device_metadata(device, device_table_id, i as u8)
                .await;
            if device_entry.entity.device_is_ready().await {
                self.device_id_lookup
                    .insert(device_entry.entity.uniq_id(), device_entry);
            }
        }

        device_table
            .metadata_crc
            .store(device_crc, Ordering::Relaxed);
        device_table
            .device_count
            .store(device_count, Ordering::Relaxed);
        warn!("Device metadata synchronized!");
        Ok(())
    }

    /// Send a command to fetch the devices from the specified device table
    async fn sync_devices(self, device_table_id: u8) -> Result<()> {
        let mut cmd = GetDevices::default();
        cmd.device_table_id = device_table_id;
        cmd.start_device_id = 0;
        cmd.max_device_request_count = 255;
        let mut devices = vec![];
        let mut device_count = 255;
        let mut device_crc = 0;

        while device_count as usize != devices.len() {
            if devices.len() > 0 {
                warn!("Received an invalid device response! Ignoring and retrying...");
                let stutter = rand::thread_rng().gen_range(800..1500);
                sleep(Duration::from_millis(stutter)).await;
                devices.clear();
            };
            let responses = self.send(cmd.clone()).await?;
            for response in responses {
                match response {
                    GetDevicesResponse::Success(data) => {
                        for device in data.devices.into_iter() {
                            devices.push(device);
                        }
                    }
                    GetDevicesResponse::SuccessComplete(data) => {
                        device_count = data.device_count;
                        device_crc = data.device_table_crc;
                    }
                    GetDevicesResponse::Failure(_) | GetDevicesResponse::FailureComplete(_) => {
                        warn!("Failure while trying to retrieve devices!");
                        device_count = 255;
                        devices.clear();
                        let stutter = rand::thread_rng().gen_range(800..1500);
                        sleep(Duration::from_millis(stutter)).await;
                    }
                }
            }
        }

        let device_table = self.get_or_add_device_table(device_table_id).await;
        for (i, device) in devices.into_iter().enumerate() {
            debug!("Found device: {:?}", device);
            let (_, device_entry) = self.get_or_add_device(device_table_id, i as u8).await;
            device_entry
                .entity
                .update_from_device_info(device, device_table_id, i as u8)
                .await;
            if device_entry.entity.device_is_ready().await {
                self.device_id_lookup
                    .insert(device_entry.entity.uniq_id(), device_entry);
            }
        }

        device_table.crc.store(device_crc, Ordering::Relaxed);
        device_table
            .device_count
            .store(device_count, Ordering::Relaxed);
        warn!("Device info synchronized!");
        Ok(())
    }

    async fn get_or_add_device_table(&self, device_table_id: u8) -> Arc<DeviceTable> {
        let val = self.device_tables.get(&device_table_id);
        match val {
            Some(dt) => dt.val().clone(),
            None => {
                let res: Arc<DeviceTable> = Default::default();
                self.device_tables.insert(device_table_id, res.clone());
                res
            }
        }
    }

    async fn get_or_add_device(
        &self,
        device_table_id: u8,
        device_id: u8,
    ) -> (bool, Arc<DeviceEntry>) {
        let table = self.get_or_add_device_table(device_table_id).await;
        let device_entry: Option<Arc<DeviceEntry>> = table
            .clone()
            .devices
            .get(&device_id)
            .map(|v| v.val().clone());
        match device_entry {
            Some(d) => (false, d),
            None => {
                let newval: Arc<DeviceEntry> = Default::default();
                table.devices.insert(device_id, newval.clone());
                (true, newval)
            }
        }
    }

    async fn set_device_state(
        &self,
        device_table: u8,
        device_id: u8,
        state: DeviceState,
    ) -> Result<()> {
        let (_, device_entry) = self.get_or_add_device(device_table, device_id).await;
        device_entry.state.store(state, Ordering::Relaxed);
        self.publish_device_state(&device_entry).await;
        Ok(())
    }

    /// This is the timer instance for polling devices and other background tasks
    async fn run_timers(self) {
        let mut t = interval(Duration::from_secs(30));
        loop {
            t.tick().await;
            let res: Result<()> = async {
                let devices = self.get_devices().await?;
                for device in devices {
                    if device.entity.device_is_ready().await {
                        self.publish_device_info(device.entity.clone()).await;
                    } else {
                        warn!(
                            "Skipped publishing info for {:?} -- device is not ready",
                            device.entity
                        );
                    }
                }
                Ok(())
            }
            .await;
            match res {
                Ok(_) => debug!("Timer cycle completed"),
                Err(e) => warn!("Error while running timer task! {:?}", e),
            };
        }
    }

    /// This is the primary run loop for the rvlink manager
    async fn run_loop(self) {
        loop {
            match self.bluetooth.recv().await {
                Ok(data) => match <events::Event as events::EventTrait>::from_payload(data) {
                    Ok(Event::CommandResponse(rsp)) => self.handle_command_response(rsp).await,
                    Ok(Event::GatewayInformation(evt)) => {
                        self.handle_gateway_information(evt).await
                    }
                    Ok(Event::TankSensorStatus(evt)) => self.handle_tank_status_update(evt).await,
                    Ok(Event::RelayBasicLatchingStatusType2(evt)) => {
                        self.handle_relay_type_2_status(evt).await
                    }
                    Ok(Event::RvStatus(evt)) => self.handle_rvstatus(evt).await,
                    Ok(Event::RealTimeClock(_))
                    | Ok(Event::DeviceSessionStatus(_))
                    | Ok(Event::DeviceOnlineStatus(_)) => { /* Irrelevant for now */ }
                    Ok(other) => info!("Received unhandled event: {:?}", other),
                    Err(e) => warn!("Failed to parse payload from bluetooth! {:?}", e),
                },
                Err(e) => warn!("Error while receiving from bluetooth! {:?}", e),
            }
        }
    }

    async fn handle_command_response(&self, rsp: CommandResponse) {
        debug!("Received Command Response: {:?}", rsp);
        let sender = self.cmdmap.get(&rsp.client_command_id);
        if let Some(sender) = sender {
            sender.val().send(rsp).unwrap_or_default();
        } else {
            warn!("Command response received with no channel to receive it!");
        }
    }

    async fn handle_gateway_information(&self, gwinfo: GatewayInformation) {
        let table_id = gwinfo.device_table_id;
        let update_device_table = match self.device_tables.get(&table_id) {
            Some(dt) => dt.val().crc.load(Ordering::Relaxed) != gwinfo.device_table_crc,
            None => true,
        };
        let update_metadata_table = match self.device_tables.get(&table_id) {
            Some(dt) => dt.val().metadata_crc.load(Ordering::Relaxed) != gwinfo.device_metadata_crc,
            None => true,
        };
        if update_device_table {
            tokio::task::spawn(self.clone().sync_devices(table_id));
        }
        if update_metadata_table {
            tokio::task::spawn(self.clone().sync_devices_metadata(table_id));
        }
    }

    async fn handle_tank_status_update(&self, tank_status: TankSensorStatus) {
        let table_id = tank_status.device_table_id;
        for status in tank_status.tank_statuses.iter() {
            self.set_device_state(
                table_id,
                status.device_id,
                DeviceState::Percentage(status.percentage),
            )
            .await
            .unwrap_or_default();
        }
    }

    async fn handle_relay_type_2_status(&self, status: RelayBasicLatchingStatusType2) {
        let table_id = status.device_table_id;
        for relay in status.relays.iter() {
            self.set_device_state(
                table_id,
                relay.device_id,
                DeviceState::Switch(relay.on_off()),
            )
            .await
            .unwrap_or_default();
        }
    }

    async fn handle_rvstatus(&self, status: RvStatus) {
        let bv = status.battery_voltage();
        if bv.is_some() {
            let newstate = DeviceState::Voltage(bv.unwrap());
            self.battery.state.store(newstate, Ordering::Relaxed);
            self.publish_device_state(&self.battery).await;
        }
    }

    pub async fn has_battery(&self) -> bool {
        self.battery.state.load(Ordering::Relaxed) != DeviceState::Unknown
    }

    pub async fn get_battery_voltage(&self) -> FixedU16<U8> {
        match self.battery.state.load(Ordering::Relaxed) {
            DeviceState::Voltage(v) => v,
            _ => Default::default(),
        }
    }

    /// Send a command to the rvlink device
    pub async fn send<T: CommandTrait>(&self, mut cmd: T) -> Result<Vec<T::ResponseType>> {
        let msgnum = self.msgnum.fetch_add(1, Ordering::SeqCst);
        let (sender, mut receiver) = mpsc::unbounded_channel();
        self.cmdmap.insert(msgnum, sender);
        // We wrap most of this in async move {} here to act similar to a finally{} block while still giving us ? operator usage
        let rsp = async move {
            debug!("Sending command# {}", msgnum);
            cmd.set_command_id(msgnum);
            self.bluetooth.send(cmd.to_payload()?).await?;
            let mut rsp: Vec<<T as CommandTrait>::ResponseType> = vec![];
            loop {
                tokio::select! {
                    data = receiver.recv() => {
                        let data = match data {
                            Some(data) => data,
                            None => {
                                return Err(AppError::Generic(
                                    "No data for command response!".to_string(),
                                ))
                            }
                        };
                        match T::ResponseType::from_payload(data.into_data()) {
                            Ok(r) => {
                                if r.complete() {
                                    rsp.push(r);
                                    break;
                                }
                                rsp.push(r);
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    _ = sleep(Duration::from_secs(15)) => {
                        return Err(AppError::Generic("Sent command timed out after 15 seconds!".into()));
                    }
                }
            }
            Ok(rsp)
        }
        .await;
        self.cmdmap.remove(&msgnum);
        rsp
    }
}
