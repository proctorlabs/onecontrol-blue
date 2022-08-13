mod hass;

pub use hass::*;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Deref, Default)]
pub struct DeviceEntity(Arc<RwLock<DeviceEntityInner>>);

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct DeviceEntityInner {
    pub device_table: u8,
    pub device_id: u8,
    pub display_name: String,
    pub typ: DeviceEntityType,
    pub location: String,
    pub base_topic: String,
    pub unit_of_measurement: Option<String>,
}

#[allow(dead_code)]
impl DeviceEntity {
    pub async fn to_discovery(&self, base_topic: String) -> HassDiscoveryInfo {
        let zelf = self.read().await;
        HassDiscoveryInfo {
            device: Some(HassDeviceInfo {
                name: crate_name!().to_string().into(),
                model: format!("{} {}", crate_name!(), crate_version!()).into(),
                manufacturer: crate_authors!().to_string().into(),
                sw_version: crate_version!().to_string().into(),
                identifiers: self.uniq_id().await.into(),
                ..Default::default()
            }),
            state_topic: "~stat".to_string().into(),
            json_attributes_topic: "~attr".to_string().into(),
            payload_available: "online".to_string().into(),
            payload_not_available: "offline".to_string().into(),
            unit_of_measurement: zelf.unit_of_measurement.clone(),
            name: zelf.display_name.clone().into(),
            icon: self.icon().await.to_string().into(),
            unique_id: self.uniq_id().await.into(),
            base_topic: base_topic.into(),
            device_class: self.device_class().await,
            ..Default::default()
        }
    }

    pub async fn device_class(&self) -> Option<String> {
        self.read().await.typ.device_class()
    }

    pub async fn icon(&self) -> &'static str {
        self.read().await.typ.icon()
    }

    pub async fn uniq_id(&self) -> String {
        let zelf = self.read().await;
        format!("CAN-{}-{}", zelf.device_table, zelf.device_id)
    }

    // pub async fn avty_topic(&self) -> String {
    //     format!("{}avty", self.node_base())
    // }

    // pub async fn stat_topic(&self) -> String {
    //     format!("{}stat", self.device_base())
    // }

    // pub async fn attr_topic(&self) -> String {
    //     format!("{}attr", self.device_base())
    // }
}

#[derive(Clone, Debug, Display, Default)]
#[allow(dead_code)]
pub enum DeviceEntityType {
    #[display(fmt = "sensor")]
    Sensor(DeviceEntitySensorClass),
    #[display(fmt = "binary_sensor")]
    BinarySensor(DeviceEntityBinarySensorClass),
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
pub enum DeviceEntitySensorClass {
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
pub enum DeviceEntityBinarySensorClass {
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

impl DeviceEntityType {
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
            Self::Sensor(c) => Some(c.to_string()),
            Self::BinarySensor(c) => Some(c.to_string()),
            _ => None,
        }
    }
}

impl DeviceEntitySensorClass {
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

impl DeviceEntityBinarySensorClass {
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
