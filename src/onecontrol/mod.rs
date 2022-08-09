use crate::bluetooth::BluetoothManager;
use crate::messages::{events, *};
use crate::*;
use dashmap::DashMap;
use rand::Rng;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Deref, Clone)]
pub struct Onecontrol(Arc<OnecontrolInner>);

#[derive(Debug)]
pub struct OnecontrolInner {
    bluetooth: BluetoothManager,
    msgnum: AtomicU16,
    cmdmap: DashMap<u16, mpsc::UnboundedSender<CommandResponse>>,
}

#[allow(dead_code)]
impl Onecontrol {
    pub async fn new(bluetooth: BluetoothManager) -> Result<Self> {
        let mut rng = rand::thread_rng();
        let msgnum = AtomicU16::new(rng.gen());
        let cmdmap = Default::default();
        Ok(Self(Arc::new(OnecontrolInner {
            bluetooth,
            msgnum,
            cmdmap,
        })))
    }

    /// Start the main loop to process incoming commands from the device
    pub async fn start(&self) -> Result<()> {
        let zelf = self.clone();
        tokio::task::spawn(async move {
            let zelf = zelf;
            loop {
                match zelf.bluetooth.recv().await {
                    Ok(data) => match <events::Event as events::EventTrait>::from_payload(data) {
                        Ok(Event::CommandResponse(rsp)) => {
                            info!("Received Command Response: {:?}", rsp);
                            if let Some(sender) = zelf.cmdmap.get(&rsp.client_command_id) {
                                sender.send(rsp).unwrap_or_default();
                            } else {
                                warn!("Command response received with no channel to receive it!");
                            }
                        }
                        Ok(other) => info!("Received: {:?}", other),
                        Err(e) => warn!("Failed to parse payload from bluetooth! {:?}", e),
                    },
                    Err(e) => warn!("Error while receiving from bluetooth! {:?}", e),
                }
            }
        });
        Ok(())
    }

    /// Send a command to the onecontrol device
    pub async fn send<T: CommandTrait>(&self, cmd: T) -> Result<Vec<T::ResponseType>> {
        let msgnum = self.msgnum.fetch_add(1, Ordering::SeqCst);
        let (sender, mut receiver) = mpsc::unbounded_channel();
        self.cmdmap.insert(msgnum, sender);
        self.bluetooth.send(cmd.to_payload()?).await?;
        let mut rsp: Vec<<T as CommandTrait>::ResponseType> = vec![];
        loop {
            let data = match receiver.recv().await {
                Some(data) => data,
                None => {
                    return Err(AppError::Generic(
                        "No data for command response!".to_string(),
                    ))
                }
            };
            match T::ResponseType::from_payload(data.into_data()) {
                Ok(r) => {
                    if r.complete() {
                        rsp.push(r);
                        break;
                    }
                    rsp.push(r);
                }
                Err(e) => return Err(e),
            }
        }
        self.cmdmap.remove(&msgnum);
        // TODO!
        Ok(rsp)
    }
}
