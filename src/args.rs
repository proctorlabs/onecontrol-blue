pub use clap::Parser;

/// Bridge for RVLink/Onecontrol devices to MQTT
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Bluetooth device name for control
    #[clap(short, long, env = "BRIDGE_DEVICE")]
    pub device: String,

    /// Log level to use [trace, debug, info, warn, error]
    #[clap(short, long, default_value = "info", env = "BRIDGE_LOG_LEVEL")]
    pub log_level: flexi_logger::LevelFilter,

    /// MQTT host
    #[clap(
        short = 'H',
        long,
        default_value = "localhost",
        env = "BRIDGE_MQTT_SERVER"
    )]
    pub host: String,

    /// MQTT port
    #[clap(short, long, default_value_t = 1883, env = "BRIDGE_MQTT_PORT")]
    pub port: u16,

    /// MQTT use SSL
    #[clap(long, env = "BRIDGE_MQTT_USE_SSL")]
    pub ssl: bool,

    /// MQTT username
    #[clap(short, long, env = "BRIDGE_MQTT_USERNAME")]
    pub username: Option<String>,

    /// MQTT password
    #[clap(short = 'P', long, env = "BRIDGE_MQTT_PASSWORD")]
    pub password: Option<String>,

    /// MQTT base topic
    #[clap(
        short = 't',
        long,
        default_value = "onecontrol-mqtt-bridge/",
        env = "BRIDGE_MQTT_BASE_TOPIC"
    )]
    pub base_topic: String,

    /// MQTT discovery topic for homeassistant (rarely needs to be changed)
    #[clap(
        long,
        default_value = "homeassistant/",
        env = "BRIDGE_MQTT_DISCOVERY_TOPIC"
    )]
    pub discovery_topic: String,
}
