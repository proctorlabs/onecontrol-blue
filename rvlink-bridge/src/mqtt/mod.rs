use crate::config;
use crate::devices::DeviceEntity;
use crate::rvlink::RVLink;
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
    rvlink: RVLink,
    client: RwLock<Option<AsyncClient>>,
    discovery_topic: String,
    base_topic: String,
    username: Option<String>,
    password: Option<String>,
    host: String,
    port: u16,
}

impl MqttManager {
    pub async fn new(rvlink: RVLink) -> Result<Self> {
        Ok(MqttManager(Arc::new(MqttManagerInner {
            rvlink,
            client: Default::default(),
            discovery_topic: config::DISCOVERY_TOPIC.clone(),
            base_topic: config::BASE_TOPIC.clone(),
            username: config::USERNAME.clone(),
            password: config::PASSWORD.clone(),
            host: config::HOST.clone(),
            port: *config::PORT,
        })))
    }

    pub async fn start(&self) -> Result<()> {
        task::spawn(self.clone().run_loop());
        Ok(())
    }

    pub async fn publish_device_info(&self, device: &DeviceEntity) -> Result<()> {
        let discovery = device.to_discovery(self.base_topic.to_string()).await;
        let config_topic = device.config_topic(&self.discovery_topic);
        self.send(
            &config_topic,
            serde_json::to_vec(&discovery)?,
            true,
            QoS::AtLeastOnce,
        )
        .await
    }

    pub async fn publish_device_state(&self, device: &DeviceEntity, state: &str) -> Result<()> {
        let state_topic = device.stat_topic(&self.base_topic);
        self.send(&state_topic, state, true, QoS::AtLeastOnce).await
    }

    async fn send<T: Into<Vec<u8>>>(
        &self,
        topic: &str,
        msg: T,
        retain: bool,
        qos: QoS,
    ) -> Result<()> {
        let client: Option<AsyncClient> = self.client.read().await.clone();
        debug!("Sending message to topic {}", topic);
        if let Some(client) = client {
            client.publish(topic, qos, retain, msg).await?;
            Ok(())
        } else {
            Err(AppError::Generic("MQTT client not ready".into()))
        }
    }

    async fn subscribe(&self, topic: &str) -> Result<()> {
        let client: Option<AsyncClient> = self.client.read().await.clone();
        if let Some(client) = client {
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
        let mut mqttoptions = MqttOptions::new("rvlink-bridge", self.host.clone(), self.port);
        if self.username.is_some() && self.password.is_some() {
            mqttoptions.set_credentials(
                self.username.clone().unwrap(),
                self.password.clone().unwrap(),
            );
        }
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        mqttoptions.set_last_will(LastWill::new(
            format!("{}avty", self.base_topic),
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
                        self.subscribe(&format!("{}+/cmd", self.base_topic)).await?;
                        self.send(
                            &format!("{}avty", self.base_topic),
                            "online",
                            true,
                            QoS::AtLeastOnce,
                        )
                        .await?;
                        Ok(())
                    }
                    Ok(Event::Incoming(Packet::Publish(pubevent))) => {
                        let topic = pubevent.topic;
                        let id = (&topic)
                            .strip_prefix(&self.base_topic.as_str())
                            .unwrap_or(&topic.as_str());
                        let id = (&id).strip_suffix("/cmd").unwrap_or(id).to_string();
                        let command = String::from_utf8(pubevent.payload.into())?;
                        let rvlink = self.rvlink.clone();
                        tokio::task::spawn(async move {
                            match rvlink.run_command(&id, &command).await {
                                Ok(_) => {}
                                Err(e) => warn!("Error when sending command! {:?}", e),
                            };
                        });
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
