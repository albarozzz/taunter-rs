use async_std::task;
use discord_presence::Client;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rcon::Connection;
use std::arch::x86_64;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};
use taunter::{helper::*, lua, Config, LastLines, Result, UsernameVictimConfig};

#[async_std::main]
async fn main() -> Result<()> {
    let config: Config = Config::new();
    let address: &str = &format!("127.0.0.1:{}", config.port);
    let user_victim_config: serde_json::Value = read_victim_config(&config);
    let unix_seconds = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("Error SystemTime before UNIX EPOCH!"),
    };
    let mut count_deaths: u32 = 0;
    let mut code: String = String::new();
    let mut dominations: HashMap<String, u64> = HashMap::new();

    if config.use_custom_lua {
        let _ = match File::open("custom.lua") {
            Ok(mut f) => f.read_to_string(&mut code),
            Err(_) => {
                panic!("file custom.lua couldn't be loaded!");
            }
        };
    }

    let soundpad_f = match File::options()
        .write(true)
        .read(true)
        .open(r"\\\\.\\pipe\\sp_remote_control")
    {
        Ok(f) => Some(f),
        Err(_) => {
            println!("Couldn't connect to soundpad named pipe");
            None
        }
    };

    let mut soundpad_pipe: Option<lua::SoundPadPipe> = match lua::SoundPadPipe::new(soundpad_f) {
        Ok(file) => Some(file),
        Err(_) => {
            println!("Couldn't connect to soundpad named pipe");
            None
        }
    };

    let mut rpc = Client::new(1198369423866744842);

    if config.use_discord_rpc {
        let _ = rpc.on_ready(|_| {
            println!("Discord Rich Presence Ready!");
        });

        println!("Waiting for Discord Rich Presence...");
        rpc.start();
        rpc.block_until_event(discord_presence::Event::Ready)?;
        println!("Connected to Discord Rich Presence!");

        let _ = rpc.set_activity(|act| {
            act.details("Playing Team Fortress 2")
                .timestamps(|t| t.start(unix_seconds))
                .state("Ready to kill!")
                .assets(|a| a.large_image("taunter"))
        });
    }

    if !config.ignore_warning {
        println!("-------------");
        println!("BEFORE DOING ANYTHING: Put these launch parameters in tf2's steam properties");
        println!("-usercon -condebug -conclearlog");
        println!("Remember to save a file named autoexec.cfg in tf2folder/tf/cfg/ with the following content:");
        println!("ip 0.0.0.0");
        println!("con_timestamp 1");
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
        let mut _interface_conn: Connection = match Connection::builder()
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
        let mut conn = lua::RconWrapper::new(_interface_conn);

        let mut last_line: String = String::from("-");
        loop {
            let file = match File::open(&console_log) {
                Err(why) => panic!("Couldn't open console log: {}", why),
                Ok(file) => file,
            };
            let ll = LastLines::new(file);
            let buffer = match ll.get_text() {
                Ok(ok) => ok,
                Err(err) => {
                    println!("Console.log with invalid UTF-8 or seeking failed: {}", err);
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
            let last_pos: usize = lines.len() - 12; // the fifth last line
            let lines_last_pos: &[&str] = &lines[last_pos..];
            // finds the index of the latest kill
            let find_line: usize = lines_last_pos
                .iter()
                .rposition(|line| *line == last_line)
                .unwrap_or(0);

            // the line of the latest kill to EOF
            for line in lines_last_pos[find_line + 1..].iter() {
                let (username, victim) = get_usernames(line);
                let mut is_dominating: bool = false;

                if username.is_empty() || victim.is_empty() {
                    continue;
                }

                if config.usernames.contains(&victim) {
                    dominations.remove(&format!("{}-{}", &username, &victim));
                    is_dominating = false;
                }

                let is_killed = match config.use_custom_lua {
                    true => {
                        lua::exec_lua_code(&mut conn, &mut soundpad_pipe, &code, &username, &victim)
                            .await?
                    }

                    false => check(
                        &config.usernames,
                        &config.username_victim,
                        &username,
                        &victim,
                    ),
                };

                if is_killed {
                    if let Some(x) = dominations.get_mut(&format!("{}-{}", &victim, &username)) {
                        *x += 1;
                    } else {
                        dominations.insert(format!("{}-{}", &victim, &username), 0);
                    }

                    //*dominations
                    //    .get_mut(&format!("{}-{}", &victim, &username))
                    //    .unwrap_or(&mut 0) += 1;
                    if *dominations
                        .get(&format!("{}-{}", &victim, &username))
                        .unwrap_or(&0)
                        == 4
                    {
                        is_dominating = true;
                    }
                    println!("{:?}", dominations);

                    count_deaths += 1;
                    println!("{}", line);
                    last_line = line.to_string();

                    #[cfg(target_family = "windows")]
                    if config.use_soundpad && !soundpad_pipe.is_none() {
                        let _ = soundpad_pipe
                            .as_mut()
                            .unwrap()
                            .write("DoPlaySoundFromCategory(1, 1)");
                    }

                    let mut individual_configuration: UsernameVictimConfig = Default::default();
                    let mut is_configured = false;
                    if let Some(value) =
                        user_victim_config.get(&format!("when_killed_{}_by_{}", &victim, &username))
                    {
                        individual_configuration = serde_json::from_value(value.clone()).unwrap();
                        is_configured = true;
                    } else if let Some(value) =
                        user_victim_config.get(&format!("when_killed_{}", &victim))
                    {
                        individual_configuration = serde_json::from_value(value.clone()).unwrap();
                        is_configured = true;
                    }

                    // if the victim is 'configured' on users.json make special commands for them.
                    if is_configured {
                        if individual_configuration.use_taunt {
                            let _ = conn.taunt(1).await;
                            //let _ = send_command(&mut conn, "taunt 1").await;
                        } else if individual_configuration.use_spinbot {
                            let _ = conn.send_command("+left").await;
                            conn.wait(1000).await;
                            let _ = conn.send_command("-left").await;
                            //let _ = send_command(&mut conn, "+left").await;
                            //task::sleep(Duration::from_millis(1000)).await;
                            //let _ = send_command(&mut conn, "-left").await;
                        }
                        if !is_dominating && !individual_configuration.message_to_send.is_empty() {
                            let choosed: &str = &individual_configuration
                                .message_to_send
                                .choose(&mut rng)
                                .unwrap()
                                .to_string()
                                .replace("{victim}", &victim)
                                .replace("{count}", &count_deaths.to_string()); // select random response
                            let _ = conn.say(choosed).await;
                        } else if is_dominating
                            && !individual_configuration
                                .message_to_send_when_dominated
                                .is_empty()
                        {
                            let choosed: &str = &individual_configuration
                                .message_to_send_when_dominated
                                .choose(&mut rng)
                                .unwrap()
                                .to_string()
                                .replace("{victim}", &victim)
                                .replace("{count}", &count_deaths.to_string()); // select random response
                            let _ = conn.say(choosed).await;
                        }
                        if !individual_configuration.extra_commands.is_empty() {
                            let _ = conn
                                .send_command(&individual_configuration.extra_commands)
                                .await;
                        }
                        continue;
                    }
                    // ------

                    // If the victim is not 'configured' on users.json proceed with generic
                    if !is_dominating && !config.words.is_empty() {
                        let choosed: &str = &config
                            .words
                            .choose(&mut rng)
                            .unwrap()
                            .to_string()
                            .replace("{victim}", &victim)
                            .replace("{count}", &count_deaths.to_string()); // select random response
                        let _ = conn.say(choosed).await;
                    } else if is_dominating && !config.special_words.is_empty() {
                        let choosed: &str = &config
                            .special_words
                            .choose(&mut rng)
                            .unwrap()
                            .to_string()
                            .replace("{victim}", &victim)
                            .replace("{count}", &count_deaths.to_string()); // select random response
                        let _ = conn.say(choosed).await;
                    }
                    if config.use_taunt {
                        let _ = conn.taunt(1).await; // select the first taunt
                    } else if config.use_spinbot {
                        let _ = conn.send_command("+left").await;
                        //task::sleep(Duration::from_millis(1000)).await;
                        conn.wait(1000).await;
                        let _ = conn.send_command("-left").await;
                    }
                    if !config.extra_commands.is_empty() {
                        let _ = conn.send_command(&config.extra_commands).await;
                    }
                    // ------

                    if config.use_discord_rpc && count_deaths % 5 == 0 {
                        let _ = rpc.set_activity(|act| {
                            act.details("Playing Team Fortress 2")
                                .timestamps(|t| t.start(unix_seconds))
                                .state(format!("Kills: {count_deaths}"))
                                .assets(|a| a.large_image("taunter"))
                        });
                    }
                }
            }
            task::sleep(Duration::from_millis(32)).await;
        }
    }
}
