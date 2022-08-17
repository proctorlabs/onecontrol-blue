mod discovery;

pub use discovery::*;

#[derive(Clone, Debug, Display, Default)]
#[allow(dead_code)]
pub enum HassDiscoveryType {
    #[display(fmt = "sensor")]
    Sensor(HassDiscoverySensorClass),
    #[display(fmt = "binary_sensor")]
    BinarySensor(HassDiscoveryBinarySensorClass),
    #[display(fmt = "media_player")]
    MediaPlayer,
    #[display(fmt = "switch")]
    #[default]
    Switch,
    #[display(fmt = "light")]
    Light,
    #[display(fmt = "thermostat")]
    Thermostat,
}

#[derive(Clone, Debug, Display, Default)]
#[allow(dead_code)]
pub enum HassDiscoverySensorClass {
    #[display(fmt = "none")]
    #[default]
    None,
    #[display(fmt = "battery")]
    Battery,
    #[display(fmt = "humidity")]
    Humidity,
    #[display(fmt = "illuminance")]
    Illuminance,
    #[display(fmt = "signal_strength")]
    SignalStrength,
    #[display(fmt = "temperature")]
    Temperature,
    #[display(fmt = "power")]
    Power,
    #[display(fmt = "pressure")]
    Pressure,
    #[display(fmt = "timestamp")]
    Timestamp,
    #[display(fmt = "current")]
    Current,
    #[display(fmt = "energy")]
    Energy,
    #[display(fmt = "power_factor")]
    PowerFactor,
    #[display(fmt = "voltage")]
    Voltage,
}

#[derive(Clone, Debug, Display, Default)]
#[allow(dead_code)]
pub enum HassDiscoveryBinarySensorClass {
    #[display(fmt = "none")]
    #[default]
    None,
    #[display(fmt = "battery")]
    Battery, // on means low, off means normal
    #[display(fmt = "battery_charging")]
    BatteryCharging, // on means charging, off means not charging
    #[display(fmt = "cold")]
    Cold, // on means cold, off means normal
    #[display(fmt = "connectivity")]
    Connectivity, // on means connected, off means disconnected
    #[display(fmt = "door")]
    Door, // on means open, off means closed
    #[display(fmt = "garage_door")]
    GarageDoor, // on means open, off means closed
    #[display(fmt = "gas")]
    Gas, // on means gas detected, off means no gas (clear)
    #[display(fmt = "heat")]
    Heat, // on means hot, off means normal
    #[display(fmt = "light")]
    Light, // on means light detected, off means no light
    #[display(fmt = "lock")]
    Lock, // on means open (unlocked), off means closed (locked)
    #[display(fmt = "moisture")]
    Moisture, // on means moisture detected (wet), off means no moisture (dry)
    #[display(fmt = "motion")]
    Motion, // on means motion detected, off means no motion (clear)
    #[display(fmt = "moving")]
    Moving, // on means moving, off means not moving (stopped)
    #[display(fmt = "occupancy")]
    Occupancy, // on means occupied, off means not occupied (clear)
    #[display(fmt = "opening")]
    Opening, // on means open, off means closed
    #[display(fmt = "plug")]
    Plug, // on means device is plugged in, off means device is unplugged
    #[display(fmt = "power")]
    Power, // on means power detected, off means no power
    #[display(fmt = "presence")]
    Presence, // on means home, off means away
    #[display(fmt = "problem")]
    Problem, // on means problem detected, off means no problem (OK)
    #[display(fmt = "safety")]
    Safety, // on means unsafe, off means safe
    #[display(fmt = "smoke")]
    Smoke, // on means smoke detected, off means no smoke (clear)
    #[display(fmt = "sound")]
    Sound, // on means sound detected, off means no sound (clear)
    #[display(fmt = "vibration")]
    Vibration, // on means vibration detected, off means no vibration (clear)
    #[display(fmt = "window")]
    Window, // on means open, off means closed
}

pub struct HassIcons;

impl HassIcons {
    pub const THERMOMETER: &'static str = "hass:thermometer";
    pub const LIGHT: &'static str = "hass:lightbulb";
    pub const EYE: &'static str = "hass:eye";
    pub const POWER: &'static str = "mdi:power";
    pub const TELEVISION: &'static str = "mdi:television";
    pub const SQUARE: &'static str = "hass:square";
    pub const WATER_PERCENT: &'static str = "hass:water-percent";
    pub const BLUETOOTH_WAVE: &'static str = "mdi:bluetooth-audio";
    pub const GARAGE: &'static str = "mdi:garage";
    pub const FLASH: &'static str = "mdi:flash";
}

impl HassDiscoveryType {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Thermostat => HassIcons::THERMOMETER,
            Self::Light => HassIcons::LIGHT,
            Self::Switch => HassIcons::POWER,
            Self::MediaPlayer => HassIcons::TELEVISION,
            Self::Sensor(c) => c.icon(),
            Self::BinarySensor(c) => c.icon(),
        }
    }

    pub fn device_class(&self) -> Option<String> {
        match self {
            Self::Sensor(HassDiscoverySensorClass::None) => None,
            Self::BinarySensor(HassDiscoveryBinarySensorClass::None) => None,
            Self::Sensor(c) => Some(c.to_string()),
            Self::BinarySensor(c) => Some(c.to_string()),
            _ => None,
        }
    }
}

impl HassDiscoverySensorClass {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Battery => HassIcons::EYE,
            Self::Humidity => HassIcons::WATER_PERCENT,
            Self::Illuminance => HassIcons::EYE,
            Self::SignalStrength => HassIcons::BLUETOOTH_WAVE,
            Self::Temperature => HassIcons::THERMOMETER,
            Self::Power => HassIcons::POWER,
            Self::Pressure => HassIcons::EYE,
            Self::Timestamp => HassIcons::EYE,
            Self::Current => HassIcons::FLASH,
            Self::Energy => HassIcons::FLASH,
            Self::PowerFactor => HassIcons::FLASH,
            Self::Voltage => HassIcons::FLASH,
            Self::None => HassIcons::EYE,
        }
    }
}

impl HassDiscoveryBinarySensorClass {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::None => HassIcons::POWER,
            Self::Battery => HassIcons::EYE,
            Self::BatteryCharging => HassIcons::EYE,
            Self::Cold => HassIcons::EYE,
            Self::Connectivity => HassIcons::EYE,
            Self::Door => HassIcons::SQUARE,
            Self::GarageDoor => HassIcons::GARAGE,
            Self::Gas => HassIcons::EYE,
            Self::Heat => HassIcons::EYE,
            Self::Light => HassIcons::EYE,
            Self::Lock => HassIcons::EYE,
            Self::Moisture => HassIcons::WATER_PERCENT,
            Self::Motion => HassIcons::EYE,
            Self::Moving => HassIcons::EYE,
            Self::Occupancy => HassIcons::EYE,
            Self::Opening => HassIcons::SQUARE,
            Self::Plug => HassIcons::EYE,
            Self::Power => HassIcons::FLASH,
            Self::Presence => HassIcons::EYE,
            Self::Problem => HassIcons::EYE,
            Self::Safety => HassIcons::EYE,
            Self::Smoke => HassIcons::EYE,
            Self::Sound => HassIcons::EYE,
            Self::Vibration => HassIcons::EYE,
            Self::Window => HassIcons::EYE,
        }
    }
}
