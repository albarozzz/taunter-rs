use super::Result;
use lazy_static::lazy_static;
use rcon::Connection;
use regex::Regex;
#[cfg(target_family = "windows")]
use std::process::Command;

pub fn check(usernames: &[String], username_victim: &[String], line: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("\"(.*?)\"").unwrap();
    }

    // the line must start with "L " to prevent fragmented lines
    // caused by reading only the 2048 bytes of the bottom of the file
    // although it is unlikely to happen
    if !line.starts_with("L ") {
        return false;
    }

    // the line we are looking for is like this:
    // L 12/12/2022 - 20:41:37: "./albarozzz<2><[U:1:383329786]><Blue>" killed "Grim Bloody Fable<3><BOT><Red>" with "force_a_nature" (attacker_position "-5301 8028 -191") (victim_position "-5053 8100 -108")
    // this captures everything in quotes and we make it a vector.
    let caps: Vec<&str> = RE
        .captures_iter(line)
        .map(|m| match m.get(1) {
            Some(i) => i.as_str(),
            None => "",
        })
        .collect();

    if caps.len() != 5 {
        return false;
    }

    let username_find = if let Some(i) = caps[0].find('<') {
        i
    } else {
        return false;
    };

    let victim_find = if let Some(i) = caps[1].find('<') {
        i
    } else {
        return false;
    };

    // eg: ./albarozzz<2><[U:1:383329786]><Blue> -> ./albarozzz
    let username = &caps[0][0..username_find];
    // eg: Grim Bloody Fable<3><BOT><Red>        -> Grim Bloody Fable
    let victim = &caps[1][0..victim_find];

    if !(usernames.contains(&username.to_owned())
        && (username_victim.contains(&victim.to_owned()) || username_victim.is_empty()))
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
