use crate::{bluetooth::BluetoothManager, onecontrol::Onecontrol, *};
use std::sync::Arc;

#[derive(Debug, Deref)]
pub struct App(Arc<AppInner>);

#[derive(Debug)]
pub struct AppInner {
    bluetooth: BluetoothManager,
    onecontrol: Onecontrol,
}

impl App {
    pub async fn new(args: Args) -> Result<Self> {
        let bluetooth = BluetoothManager::new(args.device).await?;
        let onecontrol = Onecontrol::new(bluetooth.clone()).await?;
        Ok(Self(Arc::new(AppInner {
            bluetooth,
            onecontrol,
        })))
    }

    pub async fn run(&self) -> Result<()> {
        self.bluetooth.start().await?;
        self.onecontrol.start().await?;
        tokio::signal::ctrl_c().await?;
        Ok(())
    }
}
