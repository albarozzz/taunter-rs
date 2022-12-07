use super::Result;
use rcon::Connection;
use std::process::Command;

pub fn check(usernames: &[String], username_victim: &[String], line: &str) -> bool {
    // rfind finds the index for the last ocurrance.
    // This is to avoid confusing the 'killed' or 'with' as a part
    // of the username or victim.
    // eg: Albaro killed Albaro with Albaro killed Pedorrito with Scattergun
    // username: Albaro killed Albaro with Albaro
    // victim:   Pedorrito
    // though it could fail.
    let killed = match line.rfind(" killed ") {
        Some(i) => i,
        None => return false,
    };

    let with = match line.rfind(" with ") {
        Some(i) => i,
        None => return false,
    };

    // if the line is
    // AAA killed BBB with Scattergun
    // then username will extract the string between the index 0 and the whitespace before killed.
    // and victim will take the victim's username between the length of ' killed ' plus 8 and the whitespace before with.
    // It was so simple!!
    let username = &line[0..killed];
    let victim = &line[killed + 8..with];

    if !(usernames.contains(&username.to_owned())
        && (username_victim.contains(&victim.to_owned()) || username_victim.is_empty()))
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
