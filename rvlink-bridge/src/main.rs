#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate hex_literal;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

mod app;
mod args;
mod bluetooth;
mod devices;
mod mqtt;
mod rvlink;

use args::*;
use flexi_logger::{AdaptiveFormat, Logger};
use rvlink_common::error::*;
use std::str::FromStr;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut builder = flexi_logger::LogSpecification::builder();
    builder.default(flexi_logger::LevelFilter::Info).module(
        "rvlink_bridge",
        flexi_logger::LevelFilter::from_str(&args.log_level.as_str())?,
    );
    Logger::with(builder.build())
        .adaptive_format_for_stderr(AdaptiveFormat::Default)
        .set_palette("196;208;31;8;59".into())
        .start()?;

    let app = app::App::new(args).await?;
    app.run().await?;
    Ok(())
}
