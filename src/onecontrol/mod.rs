use crate::bluetooth::BluetoothManager;
use crate::messages::{events, *};
use crate::*;
use dashmap::DashMap;
use rand::Rng;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{interval, sleep, Duration};

#[derive(Debug, Deref, Clone)]
pub struct Onecontrol(Arc<OnecontrolInner>);

// #[derive(Debug)]
// struct DeviceEntry {
//     device: Option<Device>,
//     metadata: Option<DeviceMetadata>,
// }

// #[derive(Debug)]
// struct DeviceTable {
//     devices: DashMap<u8, DeviceEntry>,
//     device_count: u8,
//     crc: u32,
//     metadata_crc: u32,
// }

#[derive(Debug)]
pub struct OnecontrolInner {
    bluetooth: BluetoothManager,
    msgnum: AtomicU16,
    cmdmap: DashMap<u16, mpsc::UnboundedSender<CommandResponse>>,
    // device_tables: DashMap<u8, DeviceTable>,
}

#[allow(dead_code)]
impl Onecontrol {
    /// Create a new Onecontrol manager instance
    pub async fn new(bluetooth: BluetoothManager) -> Result<Self> {
        let mut rng = rand::thread_rng();
        let msgnum = AtomicU16::new(rng.gen());
        let cmdmap = Default::default();
        Ok(Self(Arc::new(OnecontrolInner {
            bluetooth,
            msgnum,
            cmdmap,
        })))
    }

    /// Start the main loop to process incoming commands from the device
    pub async fn start(&self) -> Result<()> {
        tokio::task::spawn(self.clone().run_loop());
        tokio::task::spawn(self.clone().run_timers());
        Ok(())
    }

    /// Send a command to get device metadata from the specified device table
    async fn get_devices_metadata(&self, device_table_id: u8) -> Result<Vec<DeviceMetadata>> {
        let mut devices = vec![];
        let mut cmd = GetDevicesMetadata::default();
        cmd.device_table_id = device_table_id;
        cmd.start_device_id = 0;
        cmd.max_device_request_count = 255;
        let responses = self.send(cmd).await?;
        for response in responses {
            match response {
                GetDevicesMetadataResponse::Success(mut data) => {
                    devices.append(&mut data.devices);
                }
                _ => {}
            }
        }
        Ok(devices)
    }

    /// Send a command to fetch the devices from the specified device table
    async fn get_devices(&self, device_table_id: u8) -> Result<Vec<Device>> {
        let mut devices = vec![];
        let mut cmd = GetDevices::default();
        cmd.device_table_id = device_table_id;
        cmd.start_device_id = 0;
        cmd.max_device_request_count = 255;
        let responses = self.send(cmd).await?;
        for response in responses {
            match response {
                GetDevicesResponse::Success(mut data) => {
                    devices.append(&mut data.devices);
                }
                _ => {}
            }
        }
        Ok(devices)
    }

    /// This is the timer instance for polling devices and other background tasks
    async fn run_timers(self) {
        let mut t = interval(Duration::from_secs(30));
        loop {
            t.tick().await;
            match self.get_devices(1).await {
                Ok(devices) => {
                    for device in devices.iter() {
                        match device {
                            Device::Full { device_type, .. } => {
                                warn!("Found device: {}", device_type.name())
                            }
                            _ => {}
                        };
                    }
                    warn!("Total devices: {}", devices.len());
                }
                Err(e) => warn!("Error while polling devices! {:?}", e),
            };
        }
    }

    /// This is the primary run loop for the onecontrol manager
    async fn run_loop(self) {
        loop {
            match self.bluetooth.recv().await {
                Ok(data) => match <events::Event as events::EventTrait>::from_payload(data) {
                    Ok(Event::CommandResponse(rsp)) => {
                        info!("Received Command Response: {:?}", rsp);
                        if let Some(sender) = self.cmdmap.get(&rsp.client_command_id) {
                            sender.send(rsp).unwrap_or_default();
                        } else {
                            // warn!("Command response received with no channel to receive it!");
                        }
                    }
                    Ok(other) => info!("Received: {:?}", other),
                    Err(e) => warn!("Failed to parse payload from bluetooth! {:?}", e),
                },
                Err(e) => warn!("Error while receiving from bluetooth! {:?}", e),
            }
        }
    }

    /// Send a command to the onecontrol device
    pub async fn send<T: CommandTrait>(&self, mut cmd: T) -> Result<Vec<T::ResponseType>> {
        let msgnum = self.msgnum.fetch_add(1, Ordering::SeqCst);
        let (sender, mut receiver) = mpsc::unbounded_channel();
        self.cmdmap.insert(msgnum, sender);
        // We wrap most of this in async move {} here to act similar to a finally{} block while still giving us ? operator usage
        let rsp = async move {
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
