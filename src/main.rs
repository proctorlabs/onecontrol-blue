#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_more;

use args::*;
use flexi_logger::{AdaptiveFormat, Logger};
use std::str::FromStr;
use error::Result;

mod args;
mod crc;
mod bluetooth;
mod error;

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
    let teststr = format!("Hello {}!", args.device);
    let res = crc::calc(teststr.as_bytes());
    info!("CRC for `{}` is: {}", teststr, res);
    bluetooth::scan(&args.device).await?;
    Ok(())
}
