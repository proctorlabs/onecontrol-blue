use crate::args::Args;
use crate::devices::DeviceEntity;
use crate::onecontrol::Onecontrol;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet};
use rumqttc::{LastWill, QoS};
use rvlink_common::error::*;
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tokio::task;
use tokio::time;

#[derive(Debug, Deref, Clone)]
pub struct MqttManager(Arc<MqttManagerInner>);

#[derive(Debug)]
#[allow(dead_code)]
pub struct MqttManagerInner {
    onecontrol: Onecontrol,
    args: Args,
    client: RwLock<Option<AsyncClient>>,
}

impl MqttManager {
    pub async fn new(onecontrol: Onecontrol, args: Args) -> Result<Self> {
        Ok(MqttManager(Arc::new(MqttManagerInner {
            onecontrol,
            args,
            client: Default::default(),
        })))
    }

    pub async fn start(&self) -> Result<()> {
        task::spawn(self.clone().run_loop());
        Ok(())
    }

    fn base_topic(&self) -> &str {
        &self.args.base_topic
    }

    pub async fn publish_device_info(&self, device: &DeviceEntity) -> Result<()> {
        let discovery = device.to_discovery(self.base_topic().to_string()).await;
        let config_topic = device.config_topic(&self.args.discovery_topic).await;
        self.send(
            &config_topic,
            serde_json::to_vec(&discovery)?,
            true,
            QoS::AtLeastOnce,
        )
        .await
    }

    pub async fn publish_device_state(&self, device: &DeviceEntity, state: &str) -> Result<()> {
        let state_topic = device.stat_topic(&self.args.base_topic).await;
        self.send(&state_topic, state, true, QoS::AtLeastOnce).await
    }

    async fn send<T: Into<Vec<u8>>>(
        &self,
        topic: &str,
        msg: T,
        retain: bool,
        qos: QoS,
    ) -> Result<()> {
        let client = self.client.read().await;
        if let Some(client) = &*client {
            client.publish(topic, qos, retain, msg).await?;
            Ok(())
        } else {
            Err(AppError::Generic("MQTT client not ready".into()))
        }
    }

    async fn subscribe(&self, topic: &str) -> Result<()> {
        let client = self.client.read().await;
        if let Some(client) = &*client {
            client.subscribe(topic, QoS::AtMostOnce).await?;
            Ok(())
        } else {
            Err(AppError::Generic("MQTT client not ready".into()))
        }
    }

    async fn update_client(&self, client: AsyncClient) {
        *self.client.write().await = Some(client);
    }

    async fn run_loop(self) {
        info!("MQTT handler task is starting...");
        let mut mqttoptions =
            MqttOptions::new("rvlink-bridge", self.args.host.clone(), self.args.port);
        if self.args.username.is_some() && self.args.password.is_some() {
            mqttoptions.set_credentials(
                self.args.username.clone().unwrap(),
                self.args.password.clone().unwrap(),
            );
        }
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        mqttoptions.set_last_will(LastWill::new(
            format!("{}status", self.args.base_topic),
            "offline",
            QoS::AtLeastOnce,
            true,
        ));
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        self.update_client(client).await;
        loop {
            let res: Result<()> = async {
                match eventloop.poll().await {
                    Ok(Event::Incoming(Packet::ConnAck(_))) => {
                        self.subscribe(&format!("{}#", self.args.base_topic))
                            .await?;
                        self.send(
                            &format!("{}status", self.args.base_topic),
                            "online",
                            true,
                            QoS::AtLeastOnce,
                        )
                        .await?;
                        Ok(())
                    }
                    Ok(Event::Incoming(Packet::Publish(pubevent))) => {
                        info!(
                            "Received on topic {}: {}",
                            pubevent.topic,
                            String::from_utf8(pubevent.payload.into())?
                        );
                        Ok(())
                    }
                    Ok(evt) => {
                        debug!("Unhandled MQTT event: {:?}", evt);
                        Ok(())
                    }
                    Err(e) => {
                        time::sleep(Duration::from_secs(2)).await;
                        Err(e.into())
                    }
                }
            }
            .await;
            if res.is_err() {
                warn!(
                    "Error occurred in MQTT handler task: {:?}",
                    res.unwrap_err()
                );
            }
        }
    }
}
