use super::Result;
#[cfg(feature = "regex")]
use fancy_regex::Regex;
#[cfg(feature = "regex")]
use lazy_static::lazy_static;
use rcon::Connection;
use std::process::Command;

#[cfg(not(feature = "regex"))]
pub fn check(usernames: &[String], username_victim: &str, line: &str) -> bool {
    // loops through the usernames and if the line starts with that username + killed + enemy then
    // this is the line we were looking for
    for username in usernames.iter() {
        if line.starts_with(&format!("{} killed {}", username, username_victim)) {
            return true;
        }
    }
    false
}

#[cfg(feature = "regex")]
pub fn check(usernames: &[String], username_victim: &[String], line: &str) -> bool {
    //  If an attacker can control the regular expressions that will be matched against,
    //  they will be able to successfully mount a denial-of-service attack.
    //  for that reason I would have prefered using a normal regular expresion instead of using look-arounds and
    //  backtracking. If you have a better regex that doesn't use this kind of thing please tell me!
    //  Hopefully this shouldn't be so bad because steam names only have 32 characters of length.

    // https://github.com/rust-lang/regex#usage-avoid-compiling-the-same-regex-in-a-loop
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"(.*)(?=\skilled)|(?<=killed\s)(.*)(?=\swith)"#).unwrap();
    }

    // if the line is:
    // AAAAAAA killed BBBBBBB with Scattergun
    // then it will match AAAAAAA and BBBBBBB
    if !RE.is_match(line).unwrap() {
        return false;
    }
    let captured: Vec<&str> = RE
        .find_iter(line)
        .filter_map(|matched| Some(matched.unwrap().as_str()))
        .collect();

    if !(usernames.contains(&captured[0].to_owned())
        && (username_victim.contains(&captured[1].to_owned()) || username_victim.is_empty()))
    {
        return false;
    }
    true
}

pub async fn send_command(conn: &mut Connection, command: &str) -> Result<()> {
    let _ = conn.cmd(command).await?;
    Ok(())
}

pub async fn play_sound(soundpad_path: &str) -> Result<()> {
    // TODO: IMPLEMENTATION TO SOUNDUX FOR LINUX USERS??
    let _ = Command::new("cmd")
        .current_dir(soundpad_path)
        .args(["/C", "Soundpad", "-rc", "DoPlaySound(1)"])
        .spawn()
        .expect("command invoking soundpad failed!");

    Ok(())
}
