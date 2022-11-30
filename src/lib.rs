use clap::Parser;
use serde::Deserialize;
use std::error::Error;

pub mod helper;

#[derive(Deserialize, Default, Parser)]
pub struct Config {
    #[arg(
        short, 
        long, 
        required_unless_present_any(["tf2_path", "rcon_password", "usernames"]),
        help("Select the configuration file to read (JSON). eg: --config config.json")
    )]
    pub config: Option<String>,

    #[serde(default = "default_port")]
    #[arg(
        short = 'p', 
        long, 
        default_value = "27015",
        help("RCON's port, default to 27015. eg: --port 27030")
    )]
    pub port: String,

    #[serde(default)]
    #[arg(
        short = 'g', 
        long = "ignore-warning", 
        default_value_t = false,
        help("Flag to ignore the warning. eg: --ignore-warning")
    )]
    pub ignore_warning: bool,

    #[arg(
        short = 't',
        long = "tf2-path",
        required = false,
        default_value = "",
        required_unless_present("config"),
        help("TF2 root directory. This parameter is required. eg: --tf2-path ...")
    )]
    pub tf2_path: String,

    #[arg(
        short,
        long = "rcon-password",
        required = false,
        default_value = "",
        required_unless_present("config"),
        help("RCON password. This parameter is required. eg: --rcon-password ...")
    )]
    pub rcon_password: String,

    #[arg(
        short = 'u', 
        long, 
        num_args(0..),
        value_delimiter = ',',
        default_value = "", 
        required = false, 
        required_unless_present("config"),
        help("List of usernames, the delimiter is ','. This parameter is required. eg: --usernames user1,user2,...")
    )]
    pub usernames: Vec<String>,

    #[cfg(not(feature = "regex"))]
    #[serde(default)]
    #[arg(
        short = 'v', 
        long = "username-victim", 
        default_value = "",
        help("The enemy's username. Optional. eg: --username-victim Pepe")
    )]
    pub username_victim: String,

    #[cfg(feature = "regex")]
    #[serde(default)]
    #[arg(
        short = 'v', 
        long = "username-victim",
        num_args(0..),
        value_delimiter = ',',
        help("The enemy's username. Optional. eg: --username-victim Pepe")
    )]
    pub username_victim: Vec<String>,

    #[serde(default)]
    #[arg(
        short = 's', 
        long = "use-spinbot", 
        default_value_t = false,
        help("Flag to use fake spinbot. Optional. eg: --use-spinbot")
    )]
    pub use_spinbot: bool,

    #[serde(default = "use_taunt")]
    #[arg(
        short = 'n', 
        long = "use-taunt", 
        default_value_t = false,
        help("Flag to use slot 1 to taunt. Optional. eg: --use-taunt")
    )]
    pub use_taunt: bool,

    #[serde(default)]
    #[arg(
        short = 'w', 
        long, num_args(0..), 
        value_delimiter = ',',
        help("A list of words to send through chat, the delimiter is ','. Optional. eg: --words hi,hello,what's up?,...")
    )]
    pub words: Option<Vec<String>>,

    #[serde(default)]
    #[arg(
        short = 'd', 
        long = "use-soundpad", 
        default_value_t = false,
        help("Flag to use soundpad. Optional. eg: --use-soundpad"),
        requires("soundpad_path")
    )]
    pub use_soundpad: bool,

    #[serde(default)]
    #[arg(
        short = 'l', 
        long = "soundpad-path", 
        default_value = "",
        help("Path to soundpad root directory. Optional. eg: --soundpad-path")
    )]
    pub soundpad_path: String,

}

fn default_port() -> String {
    String::from("27015")
}

fn use_taunt() -> bool {
    true
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
