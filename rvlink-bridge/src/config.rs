pub use clap::Parser;

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
    pub static ref LOG_LEVEL: &'static flexi_logger::LevelFilter = &ARGS.log_level;
    pub static ref DEVICE: &'static String = &ARGS.device;
    pub static ref HOST: &'static String = &ARGS.host;
    pub static ref PORT: u16 = ARGS.port;
    pub static ref SSL: bool = ARGS.ssl;
    pub static ref USERNAME: &'static Option<String> = &ARGS.username;
    pub static ref PASSWORD: &'static Option<String> = &ARGS.password;
    pub static ref BASE_TOPIC: &'static String = &ARGS.base_topic;
    pub static ref DISCOVERY_TOPIC: &'static String = &ARGS.discovery_topic;
}

/// Bridge for RVLink/Onecontrol devices to MQTT
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Bluetooth device name for control
    #[clap(short, long, env = "RVLINK_BRIDGE_DEVICE")]
    pub device: String,

    /// Log level to use [trace, debug, info, warn, error]
    #[clap(short, long, default_value = "info", env = "RVLINK_BRIDGE_LOG_LEVEL")]
    pub log_level: flexi_logger::LevelFilter,

    /// MQTT host
    #[clap(
        short = 'H',
        long,
        default_value = "localhost",
        env = "RVLINK_BRIDGE_MQTT_SERVER"
    )]
    pub host: String,

    /// MQTT port
    #[clap(short, long, default_value_t = 1883, env = "RVLINK_BRIDGE_MQTT_PORT")]
    pub port: u16,

    /// MQTT use SSL
    #[clap(long, env = "RVLINK_BRIDGE_MQTT_USE_SSL")]
    pub ssl: bool,

    /// MQTT username
    #[clap(short, long, env = "RVLINK_BRIDGE_MQTT_USERNAME")]
    pub username: Option<String>,

    /// MQTT password
    #[clap(short = 'P', long, env = "RVLINK_BRIDGE_MQTT_PASSWORD")]
    pub password: Option<String>,

    /// MQTT base topic
    #[clap(
        short = 't',
        long,
        default_value = "rvlink-bridge/",
        env = "RVLINK_BRIDGE_MQTT_BASE_TOPIC"
    )]
    pub base_topic: String,

    /// MQTT discovery topic for homeassistant (rarely needs to be changed)
    #[clap(
        long,
        default_value = "homeassistant/",
        env = "RVLINK_BRIDGE_MQTT_DISCOVERY_TOPIC"
    )]
    pub discovery_topic: String,
}
