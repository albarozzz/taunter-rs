use async_std::task;
use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rcon::Connection;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread;
use std::time::Duration;
use taunter::{helper::*, Config, LastLines, Result};

#[async_std::main]
async fn main() -> Result<()> {
    let mut config = Config::parse();
    if let Some(config_file) = config.config {
        let json_file: BufReader<File> = match File::open(Path::new(&config_file)) {
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
        config = _config;
    };

    let address: &str = &format!("127.0.0.1:{}", config.port);

    if !config.ignore_warning {
        println!("-------------");
        println!("BEFORE DOING ANYTHING: Put these launch parameters in tf2's steam properties");
        println!("-usercon -condebug -conclearlog");
        println!("Remember to save a file named autoexec.cfg in tf2folder/tf/cfg/ with the following content:");
        println!("ip 0.0.0.0");
        println!("rcon_password {}", config.rcon_password);
        println!("net_start");
        println!("You can disable this warning in the config.json");
        println!("-------------");
        println!("Starting in 10 seconds");
        thread::sleep(Duration::from_secs(10));
        println!("Starting now");
    }

    let tf2path = Path::new(&config.tf2_path);
    let console_log = tf2path.join("tf").join("console.log");

    let mut rng = thread_rng();
    loop {
        let mut conn: Connection = match Connection::builder()
            .connect(address, &config.rcon_password)
            .await
        {
            Ok(_conn) => _conn,
            Err(why) => {
                println!("No RCON detected from {} or incorrect password", address);
                println!("{}", why);
                task::sleep(Duration::from_secs(2)).await;
                continue;
            }
        };
        let mut last_line: String = String::from("-");
        loop {
            let file = match File::open(&console_log) {
                Err(why) => panic!("couldn't open console log: {}", why),
                Ok(file) => file,
            };
            let ll = LastLines::new(file);
            let buffer = match ll.get_text() {
                Ok(ok) => ok,
                Err(err) => {
                    println!("Console.log with unvalid UTF-8 or seeking failed: {}", err);
                    task::sleep(Duration::from_secs(2)).await;
                    break;
                }
            };
            let lines: Vec<&str> = buffer.split('\n').collect(); // collect lines of the file to loop through them, this is not efficient? but I think BufReader.lines() won't work for this kind of thing?
            if lines.len() <= 6 {
                println!("Waiting for console.log to output");
                task::sleep(Duration::from_secs(1)).await;
                break;
            }
            let last_pos: usize = lines.len() - 6; // the fifth last line
            let lines_last_pos: &[&str] = &lines[last_pos..];
            // finds the index of the latest kill
            let find_line: usize = lines_last_pos
                .iter()
                .rposition(|line| *line == last_line)
                .unwrap_or(0);

            // the line of the latest kill to EOF
            for (i, line) in lines_last_pos[find_line + 1..].iter().enumerate() {
                if check(&config.usernames, &config.username_victim, line) {
                    println!("Position: {}, Line: {}", i, line);
                    last_line = line.to_string();

                    #[cfg(target_family = "windows")]
                    if config.use_soundpad {
                        let _ = play_sound(&config.soundpad_path).await;
                    }
                    if !config.words.is_empty() {
                        let choosed: &str = config.words.choose(&mut rng).unwrap(); // select random response
                        let _ = send_command(&mut conn, &format!("say {}", choosed)).await;
                    }
                    if config.use_spinbot {
                        let _ = send_command(&mut conn, "+left").await;
                        task::sleep(Duration::from_millis(1000)).await;
                        let _ = send_command(&mut conn, "-left").await;
                        continue;
                    }
                    if config.use_taunt {
                        let _ = send_command(&mut conn, "taunt 1").await; // select the first taunt
                    }
                    if !config.extra_commands.is_empty() {
                        let _ = send_command(&mut conn, &config.extra_commands).await;
                    }
                }
            }
            task::sleep(Duration::from_millis(125)).await;
        }
    }
}
