use atomic::Atomic;
use hmac::{Hmac, Mac};
use lockfree::map::Map;
use rvlink_common::devices::DeviceEntityType;
use rvlink_common::hass::*;
use rvlink_proto::{Device, DeviceMetadata, DeviceMetadataFull, DeviceType, FunctionName};
use sha2::Sha256;
use std::sync::atomic::*;
use std::sync::Arc;

type HmacSha256 = Hmac<Sha256>;

lazy_static! {
    // Using HMAC to ensure the original machine ID cannot be easily derived
    static ref MACHINEID: String = {
        let machine_uid: String = machine_uid::get().unwrap_or_else(|_| "UNKNOWN".into());
        let mut mac =
            HmacSha256::new_from_slice(b"rvlink-bridge").expect("HMAC can take key of any size");
        mac.update(machine_uid.as_bytes());
        let result = mac.finalize();
        let mac_bytes = result.into_bytes();
        let encoded_str = base64::encode(mac_bytes);
        // remove all possible symbols from base64
        encoded_str.replace(&['/', '+', '='][..], "")
    };
}

#[derive(Debug, Clone, Deref, Default)]
pub struct DeviceEntity(Arc<DeviceEntityInner>);

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct DeviceEntityInner {
    pub source: Atomic<DeviceEntitySource>,
    pub device_instance: AtomicU8,
    pub function_instance: AtomicU8,
    pub device_type: Atomic<DeviceType>,
    pub function_name: Atomic<FunctionName>,
    pub attributes: Map<String, String>,
    pub has_device_metadata: AtomicBool,
    pub has_device_info: AtomicBool,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum DeviceEntitySource {
    #[default]
    None,
    System {
        typ: SystemEntityType,
    },
    CAN {
        device_table: u8,
        device_id: u8,
    },
}

#[derive(Debug, Default, Display, Clone, Copy)]
pub enum SystemEntityType {
    #[default]
    #[display(fmt = "Battery")]
    Battery,
}

#[allow(dead_code)]
impl DeviceEntity {
    pub async fn new_system(typ: SystemEntityType) -> Self {
        let res: Self = Default::default();
        res.source
            .store(DeviceEntitySource::System { typ }, Ordering::Relaxed);
        res.has_device_metadata.store(true, Ordering::Relaxed);
        res.has_device_info.store(true, Ordering::Relaxed);
        res
    }

    pub async fn to_discovery(&self, base_topic: String) -> HassDiscoveryInfo {
        HassDiscoveryInfo {
            device: Some(HassDeviceInfo {
                name: crate_name!().to_string().into(),
                model: format!("{} {}", crate_name!(), crate_version!()).into(),
                manufacturer: crate_authors!().to_string().into(),
                sw_version: crate_version!().to_string().into(),
                identifiers: MACHINEID.to_string().into(),
                ..Default::default()
            }),
            state_topic: self.stat_topic("~").into(),
            json_attributes_topic: self.attr_topic("~").into(),
            availability_topic: self.avty_topic("~").into(),
            command_topic: self.command_topic("~"),
            base_topic: base_topic.into(),
            payload_on: "on".to_string().into(),
            payload_off: "off".to_string().into(),
            payload_open: "open".to_string().into(),
            payload_close: "close".to_string().into(),
            payload_stop: "stop".to_string().into(),
            payload_available: "online".to_string().into(),
            payload_not_available: "offline".to_string().into(),
            name: self.display_name().into(),
            icon: self.hass_icon().to_string().into(),
            unique_id: self.uniq_id().into(),
            device_class: self.hass_device_class(),
            ..Default::default()
        }
    }

    pub async fn device_is_ready(&self) -> bool {
        self.has_device_info.load(Ordering::Relaxed)
            && self.has_device_metadata.load(Ordering::Relaxed)
    }

    pub async fn update_from_device_info(
        &self,
        device_info: Device,
        device_table: u8,
        device_id: u8,
    ) {
        self.source.store(
            DeviceEntitySource::CAN {
                device_id,
                device_table,
            },
            Ordering::Relaxed,
        );
        match device_info {
            Device::Full {
                device_type,
                device_instance,
                product_id,
                mac_address,
                ..
            } => {
                self.device_type.store(device_type, Ordering::Relaxed);
                self.device_instance
                    .store(device_instance, Ordering::Relaxed);
                self.attributes
                    .insert("product_id".into(), product_id.to_string());
                self.attributes
                    .insert("device_mac".into(), mac_address.to_string());
            }
            Device::Basic { .. } | Device::None => {
                self.source
                    .store(DeviceEntitySource::None, Ordering::Relaxed);
            }
        }
        self.has_device_info.store(true, Ordering::Relaxed);
    }

    pub async fn update_from_device_metadata(
        &self,
        device_info: DeviceMetadata,
        device_table: u8,
        device_id: u8,
    ) {
        self.source.store(
            DeviceEntitySource::CAN {
                device_id,
                device_table,
            },
            Ordering::Relaxed,
        );
        match device_info {
            DeviceMetadata::Full(DeviceMetadataFull {
                function_name,
                function_instance,
                device_capabilities,
                can_version,
                circuit_number,
                software_part_number,
                ..
            }) => {
                self.function_name.store(function_name, Ordering::Relaxed);
                self.function_instance
                    .store(function_instance, Ordering::Relaxed);
                self.attributes.insert(
                    "device_capabilities".into(),
                    format!("{:#02x}", device_capabilities),
                );
                self.attributes
                    .insert("can_version".into(), format!("{:#02x}", can_version));
                self.attributes
                    .insert("circuit_number".into(), format!("{}", circuit_number));
                self.attributes.insert(
                    "software_part_number".into(),
                    software_part_number.to_string(),
                );
            }
            DeviceMetadata::Basic { .. } | DeviceMetadata::None => {
                self.source
                    .store(DeviceEntitySource::None, Ordering::Relaxed);
            }
        }
        self.has_device_metadata.store(true, Ordering::Relaxed);
    }

    pub async fn get_device_address(&self) -> Option<(u8, u8)> {
        match self.source.load(Ordering::Relaxed) {
            DeviceEntitySource::CAN {
                device_table,
                device_id,
            } => Some((device_table, device_id)),
            _ => None,
        }
    }
}

impl DeviceEntityInner {
    pub fn hass_device_class(&self) -> Option<String> {
        self.hass_device_type().device_class()
    }

    pub fn hass_icon(&self) -> &'static str {
        self.hass_device_type().icon()
    }

    pub fn display_name(&self) -> String {
        match self.source.load(Ordering::Relaxed) {
            DeviceEntitySource::None => "rvlink-bridge".into(),
            DeviceEntitySource::System { typ } => typ.to_string(),
            DeviceEntitySource::CAN { .. } => {
                if self.function_name.load(Ordering::Relaxed) != FunctionName::Unknown {
                    if self.function_instance.load(Ordering::Relaxed) > 0 {
                        format!(
                            "{} {}",
                            self.function_name.load(Ordering::Relaxed),
                            self.function_instance.load(Ordering::Relaxed)
                        )
                    } else {
                        self.function_name.load(Ordering::Relaxed).to_string()
                    }
                } else {
                    self.device_type.load(Ordering::Relaxed).to_string()
                }
            }
        }
    }

    pub fn hass_device_type(&self) -> HassDiscoveryType {
        match self
            .function_name
            .load(Ordering::Relaxed)
            .device_entity_type()
        {
            DeviceEntityType::LightSwitch => HassDiscoveryType::Light,
            DeviceEntityType::WaterHeater
            | DeviceEntityType::WaterPump
            | DeviceEntityType::DoorLock
            | DeviceEntityType::Switch => HassDiscoveryType::Switch,
            DeviceEntityType::Awning => HassDiscoveryType::Cover(HassDiscoveryCoverClass::Awning),
            DeviceEntityType::Slide => HassDiscoveryType::Cover(HassDiscoveryCoverClass::Door),
            DeviceEntityType::Battery
            | DeviceEntityType::FreshTank
            | DeviceEntityType::GreyTank
            | DeviceEntityType::BlackTank
            | DeviceEntityType::Sensor
            | DeviceEntityType::FuelTank
            | DeviceEntityType::Brakes
            | DeviceEntityType::SignalLights
            | DeviceEntityType::None
            | DeviceEntityType::LPTank => HassDiscoveryType::Sensor(HassDiscoverySensorClass::None),
            DeviceEntityType::Thermostat => HassDiscoveryType::Thermostat,
        }
    }

    pub fn uniq_id(&self) -> String {
        match self.source.load(Ordering::Relaxed) {
            DeviceEntitySource::None => "rvlink-bridge".into(),
            DeviceEntitySource::System { typ } => typ.to_string().replace(' ', "_").to_lowercase(),
            DeviceEntitySource::CAN {
                device_table,
                device_id,
            } => format!(
                "{}-{}-can-{}-{}",
                *MACHINEID,
                self.function_name
                    .load(Ordering::Relaxed)
                    .device_entity_type(),
                device_table,
                device_id
            ),
        }
    }

    pub fn avty_topic(&self, base_topic: &str) -> String {
        format!("{}avty", base_topic)
    }

    pub fn stat_topic(&self, base_topic: &str) -> String {
        format!("{}{}/stat", base_topic, self.uniq_id())
    }

    pub fn attr_topic(&self, base_topic: &str) -> String {
        format!("{}{}/attr", base_topic, self.uniq_id())
    }

    pub fn command_topic(&self, base_topic: &str) -> Option<String> {
        match self.hass_device_type() {
            HassDiscoveryType::Sensor(_) | HassDiscoveryType::BinarySensor(_) => None,
            HassDiscoveryType::Cover(_)
            | HassDiscoveryType::MediaPlayer
            | HassDiscoveryType::Switch
            | HassDiscoveryType::Light
            | HassDiscoveryType::Thermostat => {
                Some(format!("{}{}/cmd", base_topic, self.uniq_id()))
            }
        }
    }

    pub fn config_topic(&self, config_base_topic: &str) -> String {
        format!(
            "{}{}/rvlink-bridge/{}/config",
            config_base_topic,
            self.hass_device_type(),
            self.uniq_id()
        )
    }
}
