#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_more;

use args::*;
use error::Result;
use flexi_logger::{AdaptiveFormat, Logger};
use std::str::FromStr;
use commands::{GetDevices, CommandTrait, Command};

mod args;
mod bluetooth;
mod crc;
mod error;
mod commands;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut builder = flexi_logger::LogSpecification::builder();
    builder.default(flexi_logger::LevelFilter::Info).module(
        "oncontrol_bridge",
        flexi_logger::LevelFilter::from_str(&args.log_level.as_str())?,
    );
    Logger::with(builder.build())
        .adaptive_format_for_stderr(AdaptiveFormat::Default)
        .set_palette("196;208;31;8;59".into())
        .start()?;
    let mut getdevices = GetDevices::default();
    getdevices.client_command_id = 500;
    getdevices.device_table_id = 200;
    let command: Command = getdevices.into();
    info!("{:?}", command.encode()?);
    if false {
        bluetooth::scan(&args.device).await?;
    }
    Ok(())
}
