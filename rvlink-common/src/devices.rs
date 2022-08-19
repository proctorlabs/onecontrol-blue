#[derive(Debug, Display, Default, PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum DeviceEntityType {
    #[default]
    #[display(fmt = "none")]
    None,
    #[display(fmt = "switch")]
    Switch,
    #[display(fmt = "light_switch")]
    LightSwitch,
    #[display(fmt = "water_heater")]
    WaterHeater,
    #[display(fmt = "water_pump")]
    WaterPump,
    #[display(fmt = "slide")]
    Slide,
    #[display(fmt = "awning")]
    Awning,
    #[display(fmt = "battery")]
    Battery,
    #[display(fmt = "fresh_tank")]
    FreshTank,
    #[display(fmt = "grey_tank")]
    GreyTank,
    #[display(fmt = "black_tank")]
    BlackTank,
    #[display(fmt = "fuel_tank")]
    FuelTank,
    #[display(fmt = "lp_tank")]
    LPTank,
    #[display(fmt = "door_lock")]
    DoorLock,
    #[display(fmt = "thermostat")]
    Thermostat,
    #[display(fmt = "brakes")]
    Brakes,
    #[display(fmt = "signal_lights")]
    SignalLights,
    #[display(fmt = "sensor")]
    Sensor,
}
