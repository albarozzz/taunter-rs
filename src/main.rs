use async_std::task;
use rcon::{Connection, Error};
use std::error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use taunter::Config;

#[async_std::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let json_file: File = match File::open(&Path::new("config.json")) {
        Ok(file) => file,
        Err(why) => {
            panic!("Error opening file config.json: {}", why);
        }
    };

    let config: Config = match serde_json::from_reader(BufReader::new(json_file)) {
        Ok(json) => json,
        Err(why) => {
            panic!("Error parsing file config.json: {}", why);
        }
    };

    let address: &str = &format!("127.0.0.1:{}", config.port);

    if !config.ignore_warning {
        println!("-------------");
        println!("BEFORE DOING ANYTHING: Put these launch parameters in tf2's steam properties");
        println!("-usercon -condebug -conclearlog");
        println!("Remember to save a file named autoexec.cfg in tf2folder/tf/cfg/ with the following content:");
        println!("ip 0.0.0.0");
        println!("rcon_password \"{}\"", config.rcon_password);
        println!("net_start");
        println!("You can disable this warning in the config.json");
        println!("-------------");
        println!("Starting in 10 seconds");
        thread::sleep(Duration::from_secs(10));
        println!("Starting now");
    }

    let tf2path = Path::new(&config.tf2_path);
    let console_log = tf2path.join("tf").join("console.log");

    let mut usernames_parsed: Vec<String> = Vec::new();

    for username in config.usernames {
        usernames_parsed.push(username);
    }
    loop {
        let mut conn: Connection = match Connection::builder()
            .connect(address, &config.rcon_password)
            .await
        {
            Ok(_conn) => _conn,
            Err(_) => {
                println!("No RCON detected from {} or incorrect password", address);
                task::sleep(Duration::from_secs(2)).await;
                continue;
            }
        };
        let mut the_last_pos = 0usize;
        loop {
            let mut file = match File::open(&console_log) {
                Err(why) => panic!("couldn't open console log: {}", why),
                Ok(file) => file,
            };
            let mut s = String::new();
            file.read_to_string(&mut s)?;
            let lines: Vec<&str> = s.split("\n").collect();
            if lines.len() <= 6 || lines.len() < the_last_pos {
                println!("Waiting for console.log to output");
                task::sleep(Duration::from_secs(1)).await;
                break;
            }
            let last_pos: usize = lines.len() - 6; // the fifth last line

            // the fifth last line to EOF line
            for (i, line) in lines[last_pos..].iter().enumerate() {
                if last_pos > the_last_pos
                    && check(&usernames_parsed, &config.username_victim, line)
                {
                    the_last_pos = last_pos + i;
                    println!("Position: {}, Line: {}", the_last_pos, line);
                    if config.use_soundpad {
                        let _ = play_sound(&config.soundpad_path).await;
                    }
                    if !config.message.is_empty() {
                        let _ = send_command(&mut conn, &format!("say {}", config.message)).await;
                    }
                    if config.use_spinbot {
                        let _ = send_command(&mut conn, "+left").await;
                        task::sleep(Duration::from_millis(1000)).await;
                        let _ = send_command(&mut conn, "-left").await;
                        continue;
                    }
                    let _ = send_command(&mut conn, "+taunt 1").await;
                }
            }
            task::sleep(Duration::from_millis(100)).await;
        }
    }
}

fn check(usernames: &Vec<String>, username_victim: &str, line: &str) -> bool {
    for username in usernames {
        if line.starts_with(&format!("{} killed {}", username, username_victim)) {
            return true;
        }
    }
    false
}

async fn send_command(conn: &mut Connection, command: &str) -> Result<(), Error> {
    let _ = conn.cmd(command).await?;
    Ok(())
}

async fn play_sound(soundpad_path: &str) -> Result<(), Box<dyn error::Error>> {
    let _ = Command::new("cmd")
        .current_dir(soundpad_path)
        .args(["/C", "Soundpad", "-rc", "DoPlaySound(1)"])
        .spawn();

    Ok(())
}
