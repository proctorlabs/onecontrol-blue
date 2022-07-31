pub use clap::Parser;

/// LCI One Control Bridge
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Device name to connect to
    #[clap(short, long)]
    pub device: String,

    /// PIN used for pairing
    #[clap(short, long)]
    pub pin: String,

    /// Log level to use [trace, debug, info, warn, error]
    #[clap(short, long, default_value = "info")]
    pub log_level: flexi_logger::LevelFilter,
}
