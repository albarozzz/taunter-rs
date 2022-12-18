use clap::Parser;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::{Seek, SeekFrom, Read};

pub mod helper;
#[cfg(test)]
pub mod tests;

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
        short = 'i', 
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
        help("List of usernames, the delimiter is ','. This parameter is required. eg: --usernames user1, user2, \"user with spaces\", ...")
    )]
    pub usernames: Vec<String>,

    #[serde(default)]
    #[arg(
        short = 'v', 
        long = "username-victim",
        num_args(0..),
        value_delimiter = ',',
        help("List of enemy's usernames, the delimiter is ','. Optional. eg: --username-victim Pepe,Paco, \"Paco with spaces\"...")
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
        help("A list of words to send through chat, the delimiter is ','. Optional. eg: --words hi, hello, \"what's up?\",...")
    )]
    pub words: Option<Vec<String>>,

    #[serde(default)]
    #[cfg(target_family = "windows")]
    #[arg(
        short = 'd', 
        long = "use-soundpad", 
        default_value_t = false,
        help("Flag to use soundpad. Optional. eg: --use-soundpad"),
        requires("soundpad_path")
    )]
    pub use_soundpad: bool,

    #[serde(default)]
    #[cfg(target_family = "windows")]
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

pub struct LastLines {
    chunk_size: u64,
    start_pos: u64,
    file: File
}

impl LastLines {
    /// Constructs an object to extract the last 2048 characters of a file without reading all of the file. if you
    /// want to change the number of characters `with_chunk_size` allows you to modify it.
    /// 
    /// file is the object File to extract the text from.
    /// 
    pub fn new(file: File) -> Self {
        let file_size = file.metadata().unwrap().len();
        Self {
            chunk_size: 2048,
            start_pos: if file_size < 2048 { 0 } else { file_size - 2048 },
            file
        }
    }

    /// Allows you to change the number of characters to read from the bottom of the file.
    /// 
    /// chunk_size is the size in bytes of the text to extract.
    pub fn with_chunk_size(mut self, chunk_size: u64) -> Self {
        let file_size = self.file.metadata().unwrap().len();
        self.chunk_size = chunk_size;
        self.start_pos = if file_size < chunk_size { 0 } else { file_size - chunk_size };
        self
    }

    /// Gets the text of size chunk_size starting in start_pos (file_size - chunk_size) 
    pub fn get_text(mut self) -> Result<String> {
        let mut buffer = String::new();
        match self.file.seek(SeekFrom::Start(self.start_pos)) {
            Ok(_) => (),
            Err(err) => return Err(Box::new(err))
        }
        match (&self.file).take(self.chunk_size).read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(err) => return Err(Box::new(err))
        }
        Ok(buffer)
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
