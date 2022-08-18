use crate::config;
use crate::{bluetooth::BluetoothManager, mqtt::MqttManager, rvlink::RVLink, *};
use std::sync::Arc;

#[derive(Debug, Deref)]
pub struct App(Arc<AppInner>);

#[derive(Debug)]
pub struct AppInner {
    bluetooth: BluetoothManager,
    rvlink: RVLink,
    mqtt: MqttManager,
}

impl App {
    pub async fn new() -> Result<Self> {
        let bluetooth = BluetoothManager::new(config::DEVICE.clone()).await?;
        let rvlink = RVLink::new(bluetooth.clone()).await?;
        let mqtt = MqttManager::new(rvlink.clone()).await?;
        rvlink.set_mqtt_manager(mqtt.clone()).await;
        Ok(Self(Arc::new(AppInner {
            bluetooth,
            rvlink,
            mqtt,
        })))
    }

    pub async fn run(&self) -> Result<()> {
        self.bluetooth.start().await?;
        self.rvlink.start().await?;
        self.mqtt.start().await?;
        tokio::signal::ctrl_c().await?;
        Ok(())
    }
}
