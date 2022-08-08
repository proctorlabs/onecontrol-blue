#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate hex_literal;

use args::*;
use bluetooth::BluetoothManager;
use error::Result;
use flexi_logger::{AdaptiveFormat, Logger};
use std::str::FromStr;

mod args;
mod bluetooth;
mod encoding;
mod error;
mod messages;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut builder = flexi_logger::LogSpecification::builder();
    builder.default(flexi_logger::LevelFilter::Info).module(
        "onecontrol_bridge",
        flexi_logger::LevelFilter::from_str(&args.log_level.as_str())?,
    );
    Logger::with(builder.build())
        .adaptive_format_for_stderr(AdaptiveFormat::Default)
        .set_palette("196;208;31;8;59".into())
        .start()?;

    let bluetooth = BluetoothManager::new(args.device.clone()).await?;
    bluetooth.start().await?;
    tokio::task::spawn(async move {
        loop {
            let dat = bluetooth.recv().await.unwrap();
            info!("Received {:?}", dat);
        }
    });
    tokio::signal::ctrl_c().await?;
    Ok(())
}
