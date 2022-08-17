#[derive(Debug, Default, PartialEq)]
#[allow(dead_code)]
pub enum DeviceEntityType {
    #[default]
    None,
    Switch,
    LightSwitch,
    WaterHeater,
    WaterPump,
    Slide,
    Awning,
    Battery,
    FreshTank,
    GreyTank,
    BlackTank,
    FuelTank,
    LPTank,
    DoorLock,
    Thermostat,
    Brakes,
    SignalLights,
    Sensor,
}
