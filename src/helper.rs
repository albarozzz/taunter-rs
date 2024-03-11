#[cfg(target_family = "windows")]
use crate::lua;
use crate::Config;
use discord_presence::Client;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
#[cfg(target_family = "windows")]
use std::io::Write;
use std::path::Path;

pub fn get_usernames(line: &str) -> (String, String) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^(\d\d/\d\d/\d\d\d\d) - (\d\d:\d\d:\d\d): (?P<user>.*) killed (?P<victim>.*) with (.*)$"
        )
        .unwrap();
    }

    // the line we are looking for is like this:
    // 03/11/2023 - 01:55:04: ./albarozzz killed Mentlegen with force_a_nature.
    let caps = match RE.captures(line) {
        Some(i) => i,
        None => return ("".to_owned(), "".to_owned()),
    };

    let username = if let Some(i) = caps.name("user") {
        i.as_str().to_owned()
    } else {
        "".to_owned()
    };

    let victim = if let Some(i) = caps.name("victim") {
        i.as_str().to_owned()
    } else {
        "".to_owned()
    };

    (username, victim)
}

pub fn check(
    usernames: &[String],
    username_victim: &[String],
    username: &str,
    victim: &str,
) -> bool {
    if (!usernames.contains(&username.to_owned()) && !usernames.is_empty())
        || (!username_victim.contains(&victim.to_owned()) && !username_victim.is_empty())
    {
        return false;
    }
    true
}

pub async fn update_rpc(
    rpc: &mut Client,
    username: &str,
    victim: &str,
    count: u32,
    start: u64,
) -> bool {
    rpc.set_activity(|act| {
        act.details("Playing Team Fortress 2")
            .timestamps(|t| t.start(start))
            .state(format!("Kills: {count} - {victim} killed by {username}"))
            .assets(|a| a.large_image("taunter"))
    })
    .is_ok()
}

pub fn read_victim_config(config: &Config) -> serde_json::Value {
    if let Some(config_file) = &config.user_victim_config {
        let json_file: BufReader<File> = match File::open(Path::new(&config_file)) {
            Ok(file) => BufReader::new(file),
            Err(why) => {
                panic!("Error opening file users.json: {}", why);
            }
        };

        let _config: serde_json::Value = match serde_json::from_reader(json_file) {
            Ok(json) => json,
            Err(why) => {
                panic!("Error parsing file users.json: {}", why);
            }
        };
        return _config;
    }
    Default::default()
}
