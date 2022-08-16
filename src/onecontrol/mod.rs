use crate::bluetooth::BluetoothManager;
use crate::devices::DeviceEntity;
use crate::messages::{events, *};
use crate::mqtt::MqttManager;
use crate::*;
use dashmap::DashMap;
use fixed::{types::extra::U8, FixedU16};
use rand::Rng;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{interval, sleep, Duration};

#[derive(Debug, Deref, Clone)]
pub struct Onecontrol(Arc<OnecontrolInner>);

#[derive(Debug, Default, Clone, Copy)]
pub enum DeviceState {
    #[default]
    Unknown,
    Switch(OnOff),
    Percentage(u8),
}

impl DeviceState {
    pub fn state_string(&self) -> String {
        match self {
            DeviceState::Unknown => "unknown".into(),
            DeviceState::Switch(onoff) => onoff.to_string(),
            DeviceState::Percentage(pc) => format!("{}%", pc),
        }
    }
}

#[derive(Debug, Default)]
pub struct DeviceEntry {
    pub entity: DeviceEntity,
    pub state: RwLock<DeviceState>,
}

#[derive(Debug, Default)]
struct DeviceTable {
    pub devices: Arc<DashMap<u8, Arc<DeviceEntry>>>,
    pub device_count: RwLock<u8>,
    pub crc: RwLock<u32>,
    pub metadata_crc: RwLock<u32>,
}

#[derive(Debug)]
pub struct OnecontrolInner {
    bluetooth: BluetoothManager,
    mqtt: RwLock<Option<MqttManager>>,
    msgnum: AtomicU16,
    cmdmap: DashMap<u16, mpsc::UnboundedSender<CommandResponse>>,
    device_tables: DashMap<u8, Arc<DeviceTable>>,
    battery_voltage: RwLock<Option<FixedU16<U8>>>,
    external_temperature: RwLock<Option<FixedU16<U8>>>,
}

#[allow(dead_code)]
impl Onecontrol {
    /// Create a new Onecontrol manager instance
    pub async fn new(bluetooth: BluetoothManager) -> Result<Self> {
        let mut rng = rand::thread_rng();
        let msgnum = AtomicU16::new(rng.gen());
        Ok(Self(Arc::new(OnecontrolInner {
            bluetooth,
            msgnum,
            cmdmap: Default::default(),
            device_tables: Default::default(),
            battery_voltage: Default::default(),
            external_temperature: Default::default(),
            mqtt: Default::default(),
        })))
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

    pub async fn get_devices(&self) -> Result<Vec<Arc<DeviceEntry>>> {
        let mut result = vec![];
        for table in self.device_tables.iter() {
            for device in table.devices.iter() {
                result.push(device.clone());
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
        let responses = self.send(cmd).await?;
        let device_table = self.get_device_table_defaulted(device_table_id);
        let mut next_device_id = 0;
        for response in responses {
            match response {
                GetDevicesMetadataResponse::Success(data) => {
                    for device in data.devices {
                        let (is_new, device_entry) =
                            self.get_device_defaulted(device_table_id, next_device_id);
                        device_entry
                            .entity
                            .update_from_device_metadata(device, device_table_id, next_device_id)
                            .await;
                        if is_new {
                            self.get_mqtt()
                                .await
                                .publish_device_info(&device_entry.entity)
                                .await
                                .unwrap_or_default();
                        }
                        next_device_id += 1;
                    }
                }
                GetDevicesMetadataResponse::SuccessComplete(data) => {
                    *device_table.metadata_crc.write().await = data.device_metadata_table_crc;
                    *device_table.device_count.write().await = data.device_count;
                }
                _ => {}
            }
        }
        warn!("Device metadata synchronized!");
        Ok(())
    }

    /// Send a command to fetch the devices from the specified device table
    async fn sync_devices(self, device_table_id: u8) -> Result<()> {
        let mut cmd = GetDevices::default();
        cmd.device_table_id = device_table_id;
        cmd.start_device_id = 0;
        cmd.max_device_request_count = 255;
        let responses = self.send(cmd).await?;
        let device_table = self.get_device_table_defaulted(device_table_id);
        let mut next_device_id = 0;
        for response in responses {
            match response {
                GetDevicesResponse::Success(data) => {
                    for device in data.devices {
                        let (is_new, device_entry) =
                            self.get_device_defaulted(device_table_id, next_device_id);
                        device_entry
                            .entity
                            .update_from_device_info(device, device_table_id, next_device_id)
                            .await;
                        if is_new {
                            self.get_mqtt()
                                .await
                                .publish_device_info(&device_entry.entity)
                                .await
                                .unwrap_or_default();
                        }
                        next_device_id += 1;
                    }
                }
                GetDevicesResponse::SuccessComplete(data) => {
                    *device_table.crc.write().await = data.device_table_crc;
                }
                _ => {}
            }
        }
        warn!("Device data synchronized!");
        Ok(())
    }

    fn get_device_table_defaulted(&self, device_table_id: u8) -> Arc<DeviceTable> {
        if !self.device_tables.contains_key(&device_table_id) {
            self.device_tables
                .insert(device_table_id, Default::default());
        }
        self.device_tables.get(&device_table_id).unwrap().clone()
    }

    fn get_device_defaulted(&self, device_table_id: u8, device_id: u8) -> (bool, Arc<DeviceEntry>) {
        let table = self.get_device_table_defaulted(device_table_id);
        match table.devices.clone().get(&device_id) {
            Some(d) => (false, (*&d).clone()),
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
        let (_, device_entry) = self.get_device_defaulted(device_table, device_id);
        let state_str = state.state_string();
        *device_entry.state.write().await = state;
        self.get_mqtt()
            .await
            .publish_device_state(&device_entry.entity, &state_str)
            .await?;
        Ok(())
    }

    /// This is the timer instance for polling devices and other background tasks
    async fn run_timers(self) {
        let mut t = interval(Duration::from_secs(30));
        loop {
            t.tick().await;
            warn!("Current state: {:?}", self.device_tables);
        }
    }

    /// This is the primary run loop for the onecontrol manager
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
                    Ok(Event::RealTimeClock(_)) | Ok(Event::DeviceSessionStatus(_)) => {
                        /* Irrelevant for now */
                    }
                    Ok(other) => info!("Received unhandled event: {:?}", other),
                    Err(e) => warn!("Failed to parse payload from bluetooth! {:?}", e),
                },
                Err(e) => warn!("Error while receiving from bluetooth! {:?}", e),
            }
        }
    }

    async fn handle_command_response(&self, rsp: CommandResponse) {
        debug!("Received Command Response: {:?}", rsp);
        if let Some(sender) = self.cmdmap.get(&rsp.client_command_id) {
            sender.send(rsp).unwrap_or_default();
        } else {
            warn!("Command response received with no channel to receive it!");
        }
    }

    async fn handle_gateway_information(&self, gwinfo: GatewayInformation) {
        let table_id = gwinfo.device_table_id;
        let update_device_table = match self.device_tables.get(&table_id) {
            Some(dt) => *dt.crc.read().await != gwinfo.device_table_crc,
            None => true,
        };
        let update_metadata_table = match self.device_tables.get(&table_id) {
            Some(dt) => *dt.metadata_crc.read().await != gwinfo.device_metadata_crc,
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
        let et = status.external_temperature();
        if bv.is_some() {
            *self.battery_voltage.write().await = bv;
        }
        if et.is_some() {
            *self.external_temperature.write().await = et;
        }
    }

    pub async fn has_battery(&self) -> bool {
        self.battery_voltage.read().await.is_some()
    }

    pub async fn get_battery_voltage(&self) -> FixedU16<U8> {
        (*self.battery_voltage.read().await).unwrap_or_default()
    }

    pub async fn has_external_temperature(&self) -> bool {
        self.external_temperature.read().await.is_some()
    }

    pub async fn get_external_temperature(&self) -> FixedU16<U8> {
        (*self.external_temperature.read().await).unwrap_or_default()
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
