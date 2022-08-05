use crate::encoding::COBS;
use crate::error::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[allow(dead_code)]
#[derive(Debug, TryFromPrimitive, IntoPrimitive, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum EventType {
    GatewayInformation = 1,
    DeviceCommand = 2,
    DeviceOnlineStatus = 3,
    DeviceLockStatus = 4,
    RelayBasicLatchingStatusType1 = 5,
    RelayBasicLatchingStatusType2 = 6,
    RvStatus = 7,
    DimmableLightStatus = 8,
    RgbLightStatus = 9,
    GeneratorGenieStatus = 10,
    HvacStatus = 11,
    TankSensorStatus = 12,
    RelayHBridgeMomentaryStatusType1 = 13,
    RelayHBridgeMomentaryStatusType2 = 14,
    HourMeterStatus = 15,
    Leveler4DeviceStatus = 16,
    LevelerConsoleText = 17,
    Leveler1DeviceStatus = 18,
    Leveler3DeviceStatus = 19,
    DeviceSessionStatus = 26,
    RealTimeClock = 32,
    CloudGatewayStatus = 33,
    TemperatureSensorStatus = 34,
    JaycoTbbStatus = 35,
    MonitorPanelStatus = 43,
    AccessoryGatewayStatus = 44,
    AwningSensorStatus = 47,
    BrakingSystemStatus = 48,
    BatteryMonitorStatus = 49,
    DoorLockStatus = 51,
    HostDebug = 102,
}

#[derive(Debug, Deref)]
pub struct EventWrapper(pub Vec<u8>);

#[allow(dead_code)]
impl EventWrapper {
    pub fn decode(data: &[u8]) -> Result<Self> {
        let data = COBS::decode(data)?;
        if data.len() < 1 {
            Err(AppError::InvalidPayload)
        } else {
            Ok(EventWrapper(data))
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        COBS::encode(&self)
    }

    pub fn event_type(&self) -> Result<EventType> {
        Ok(EventType::try_from(self[0])?)
    }
}
