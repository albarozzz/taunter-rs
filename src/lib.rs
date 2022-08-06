use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: String,
    #[serde(default)]
    pub ignore_warning: bool,
    pub tf2_path: String,
    pub rcon_password: String,
    pub usernames: Vec<String>,
    #[serde(default)]
    pub username_victim: String,
    #[serde(default)]
    pub use_spinbot: bool,
    #[serde(default)]
    pub words: Vec<String>,
    #[serde(default)]
    pub use_soundpad: bool,
    #[serde(default)]
    pub soundpad_path: String,
}

fn default_port() -> String {
    String::from("27015")
}
