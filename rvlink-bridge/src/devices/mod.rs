use hmac::{Hmac, Mac};
use rvlink_common::devices::DeviceEntityType;
use rvlink_common::hass::*;
use rvlink_proto::{Device, DeviceMetadata, DeviceMetadataFull, DeviceType, FunctionName};
use sha2::Sha256;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

type HmacSha256 = Hmac<Sha256>;

lazy_static! {
    static ref MACHINEID: String = {
        let machine_uid: String = machine_uid::get().unwrap_or_else(|_| "UNKNOWN".into());
        let mut mac =
            HmacSha256::new_from_slice(b"rvlink-bridge").expect("HMAC can take key of any size");
        mac.update(machine_uid.as_bytes());
        let result = mac.finalize();
        let mac_bytes = result.into_bytes();
        base64::encode(mac_bytes)
    };
}

#[derive(Debug, Clone, Deref, Default)]
pub struct DeviceEntity(Arc<RwLock<DeviceEntityInner>>);

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct DeviceEntityInner {
    pub source: DeviceEntitySource,
    pub display_name: String,
    pub typ: DeviceEntityType,
    pub device_instance: u8,
    pub function_instance: u8,
    pub device_type: DeviceType,
    pub function_name: FunctionName,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub enum DeviceEntitySource {
    #[default]
    None,
    System {
        name: String,
    },
    CAN {
        device_table: u8,
        device_id: u8,
    },
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
                identifiers: MACHINEID.to_string().into(),
                ..Default::default()
            }),
            state_topic: zelf.stat_topic(&base_topic).into(),
            json_attributes_topic: zelf.attr_topic(&base_topic).into(),
            availability_topic: zelf.avty_topic(&base_topic).into(),
            payload_available: "online".to_string().into(),
            payload_not_available: "offline".to_string().into(),
            // unit_of_measurement: zelf.unit_of_measurement.clone(),
            name: zelf.display_name.clone().into(),
            icon: zelf.hass_icon().to_string().into(),
            unique_id: zelf.uniq_id().into(),
            base_topic: base_topic.into(),
            device_class: zelf.hass_device_class(),
            ..Default::default()
        }
    }

    pub async fn update_from_device_info(
        &self,
        device_info: Device,
        device_table: u8,
        device_id: u8,
    ) {
        let mut device = self.write().await;
        device.source = DeviceEntitySource::CAN {
            device_id,
            device_table,
        };
        match device_info {
            Device::Full {
                device_type,
                device_instance,
                product_id,
                mac_address,
                ..
            } => {
                device.device_type = device_type;
                device.device_instance = device_instance;
                device
                    .attributes
                    .insert("product_id".into(), product_id.to_string());
                device
                    .attributes
                    .insert("device_mac".into(), mac_address.to_string());
            }
            Device::Basic { .. } | Device::None => {
                device.source = DeviceEntitySource::None;
            }
        }
    }

    pub async fn update_from_device_metadata(
        &self,
        device_info: DeviceMetadata,
        device_table: u8,
        device_id: u8,
    ) {
        let mut device = self.write().await;
        device.source = DeviceEntitySource::CAN {
            device_id,
            device_table,
        };
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
                device.function_name = function_name;
                device.function_instance = function_instance;
                device.display_name = if function_instance > 0 {
                    format!("{} {}", function_name, function_instance)
                } else {
                    function_name.to_string()
                };
                device.attributes.insert(
                    "device_capabilities".into(),
                    format!("{:#02x}", device_capabilities),
                );
                device
                    .attributes
                    .insert("can_version".into(), format!("{:#02x}", can_version));
                device
                    .attributes
                    .insert("circuit_number".into(), format!("{}", circuit_number));
                device.attributes.insert(
                    "software_part_number".into(),
                    software_part_number.to_string(),
                );
            }
            DeviceMetadata::Basic { .. } | DeviceMetadata::None => {
                device.display_name = "unknown".into();
                device.source = DeviceEntitySource::None;
            }
        }
    }

    pub async fn config_topic(&self, config_base_topic: &str) -> String {
        self.read().await.config_topic(config_base_topic)
    }

    pub async fn stat_topic(&self, base_topic: &str) -> String {
        self.read().await.stat_topic(base_topic)
    }
}

impl DeviceEntityInner {
    pub fn hass_device_class(&self) -> Option<String> {
        self.hass_device_type().device_class()
    }

    pub fn hass_icon(&self) -> &'static str {
        self.hass_device_type().icon()
    }

    pub fn hass_device_type(&self) -> HassDiscoveryType {
        match self.function_name.device_entity_type() {
            DeviceEntityType::LightSwitch => HassDiscoveryType::Light,
            DeviceEntityType::WaterHeater
            | DeviceEntityType::WaterPump
            | DeviceEntityType::Slide
            | DeviceEntityType::DoorLock
            | DeviceEntityType::Switch
            | DeviceEntityType::Awning => HassDiscoveryType::Switch,
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
        match &self.source {
            DeviceEntitySource::None => "unknown".into(),
            DeviceEntitySource::System { name } => name.clone(),
            DeviceEntitySource::CAN {
                device_table,
                device_id,
            } => format!("CAN-{}-{}", device_table, device_id),
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

    pub fn config_topic(&self, config_base_topic: &str) -> String {
        match self.hass_device_type() {
            _ => format!(
                "{}sensor/{}/device/config",
                config_base_topic,
                self.uniq_id()
            ),
        }
    }
}
