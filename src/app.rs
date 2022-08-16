use crate::{bluetooth::BluetoothManager, mqtt::MqttManager, onecontrol::Onecontrol, *};
use std::sync::Arc;

#[derive(Debug, Deref)]
pub struct App(Arc<AppInner>);

#[derive(Debug)]
pub struct AppInner {
    bluetooth: BluetoothManager,
    onecontrol: Onecontrol,
    mqtt: MqttManager,
}

impl App {
    pub async fn new(args: Args) -> Result<Self> {
        let bluetooth = BluetoothManager::new(args.device.clone()).await?;
        let onecontrol = Onecontrol::new(bluetooth.clone()).await?;
        let mqtt = MqttManager::new(onecontrol.clone(), args).await?;
        onecontrol.set_mqtt_manager(mqtt.clone()).await;
        Ok(Self(Arc::new(AppInner {
            bluetooth,
            onecontrol,
            mqtt,
        })))
    }

    pub async fn run(&self) -> Result<()> {
        self.bluetooth.start().await?;
        self.onecontrol.start().await?;
        self.mqtt.start().await?;
        tokio::signal::ctrl_c().await?;
        Ok(())
    }
}
