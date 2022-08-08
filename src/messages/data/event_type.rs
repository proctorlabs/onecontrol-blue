use super::*;

#[allow(dead_code)]
#[derive(Default, Debug, Display, TryFromPrimitive, IntoPrimitive, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum EventType {
    #[default]
    Unknown = 0,
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
