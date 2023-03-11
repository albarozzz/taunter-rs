use super::Result;
use lazy_static::lazy_static;
use rcon::Connection;
use regex::Regex;
#[cfg(target_family = "windows")]
use std::process::Command;

pub fn check(usernames: &[String], username_victim: &[String], line: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(\d\d/\d\d/\d\d\d\d) - (\d\d:\d\d:\d\d): (?P<user>.*) killed (?P<victim>.*) with (.*)"
        )
        .unwrap();
    }

    // the line we are looking for is like this:
    // 03/11/2023 - 01:55:04: ./albarozzz killed Mentlegen with force_a_nature.
    let caps = match RE.captures(line) {
        Some(i) => i,
        None => return false,
    };

    let username = if let Some(i) = caps.name("user") {
        i.as_str()
    } else {
        return false;
    };

    let victim = if let Some(i) = caps.name("victim") {
        i.as_str()
    } else {
        return false;
    };

    if (!usernames.contains(&username.to_owned()) && !usernames.is_empty())
        || (!username_victim.contains(&victim.to_owned()) && !username_victim.is_empty())
    {
        return false;
    }
    true
}

pub async fn send_command(conn: &mut Connection, command: &str) -> Result<()> {
    // TODO: EXECUTE CFG files to customize what to do
    let _ = conn.cmd(command).await?;
    Ok(())
}

#[cfg(target_family = "windows")]
pub async fn play_sound(soundpad_path: &str) -> Result<()> {
    let _ = Command::new("cmd")
        .current_dir(soundpad_path)
        .args(["/C", "Soundpad", "-rc", "DoPlaySound(1)"])
        .spawn()
        .expect("command invoking soundpad failed!");

    Ok(())
}

#[cfg(target_family = "unix")]
pub async fn play_sound(_soundpad_path: &str) -> Result<()> {
    // TODO: IMPLEMENTATION TO SOUNDUX FOR LINUX USERS
    Ok(())
}
