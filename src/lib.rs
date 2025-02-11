use clap::Parser;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::{Seek, SeekFrom, Read, BufReader};
use std::path::Path;
pub mod helper;
pub mod lua;
#[cfg(test)]
pub mod tests;

#[derive(Deserialize, Default, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(
        short = 'U', 
        long, 
        help("Select the victim configuration file to read (JSON). eg: --user-victim-config users.json")
    )]
    pub user_victim_config: Option<String>,

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
        //required_unless_present("config"),
        help("TF2 root directory. This parameter is required. eg: --tf2-path ...")
    )]
    pub tf2_path: String,

    #[arg(
        short,
        long = "rcon-password",
        required = false,
        default_value = "",
        //required_unless_present("config"),
        help("RCON password. This parameter is required. eg: --rcon-password ...")
    )]
    pub rcon_password: String,

    #[serde(default)]
    #[arg(
        short = 'x', 
        long = "extra-commands", 
        default_value = "",
        help("Extra commands (separated by ';') to send through rcon. Optional. eg: --extra-commands 'play_sound ...; ...'")
    )]
    pub extra_commands: String,

    #[arg(
        short = 'u', 
        long, 
        num_args(0..),
        value_delimiter = ',',
        default_value = "", 
        required = false, 
        //required_unless_present("config"),
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
        long, 
        num_args(0..),
        value_delimiter = ',',
        help("A list of words to send through chat, the delimiter is ','. Optional. eg: --words hi, hello, \"what's up?\",...")
    )]
    pub words: Vec<String>,

    #[serde(default)]
    #[arg(
        short = 'a', 
        long = "special-words", 
        num_args(0..),
        value_delimiter = ',',
        help("A list of words to send through chat when victim is dominated by any username selected, the delimiter is ','. Optional. eg: --special-words hi, hello, \"what's up?\",...")
    )]
    pub special_words: Vec<String>,

    #[serde(default)]
    #[arg(
        short = 'o', 
        long = "use-discord-rpc", 
        default_value_t = false,
        help("Flag to use discord rich presence. Optional. eg: --use-discord-rpc")
    )]
    pub use_discord_rpc: bool,

    #[serde(default)]
    #[arg(
        short = 'l', 
        long = "use-custom-lua", 
        default_value_t = false,
        help("Flag to use lua custom code. Optional. eg: --use-custom-lua")
    )]
    pub use_custom_lua: bool,

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

}

fn default_port() -> String {
    String::from("27015")
}

fn use_taunt() -> bool {
    true
}

impl Config {
    pub fn new() -> Self {
        
        if std::env::args().count() != 1 {
            println!("Parsing arguments...");
            return Config::parse();
        }

        // if arguments were not passed try to load config.json
        let json_file: BufReader<File> = match File::open(Path::new("config.json")) {
            Ok(file) => BufReader::new(file),
            Err(why) => {
                panic!("Error opening file config.json: {}", why);
            }
        };

        let _config: Config = match serde_json::from_reader(json_file) {
            Ok(json) => json,
            Err(why) => {
                panic!("Error parsing file config.json: {}", why);
            }
        };
        _config
    }

}

#[derive(Deserialize, Default)]
pub struct UsernameVictimConfig {
    // mutually exclusive
    #[serde(default)]
    pub use_taunt: bool,
    #[serde(default)]
    pub use_spinbot: bool,
    // -------------------
    #[serde(default)]
    pub message_to_send: Vec<String>,
    #[serde(default)]
    pub message_to_send_when_dominated: Vec<String>,
    #[serde(default)]
    pub extra_commands: String
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
        let mut buffer: Vec<u8> = Vec::new();
        self.file.seek(SeekFrom::Start(self.start_pos))?;
        (&self.file).take(self.chunk_size).read_to_end(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
