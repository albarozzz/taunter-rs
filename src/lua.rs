use mlua::{Function, Lua, UserData};

use crate::Result;
use async_std::task;
use core::panic;
use rcon::Connection;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

pub struct SoundPadPipe {
    pub pipe: File,
}

impl SoundPadPipe {
    pub fn new(f: Option<File>) -> Result<Self> {
        match f {
            Some(sp) => Ok(Self { pipe: sp }),
            None => Err("File not found!".into()),
        }
    }

    pub fn write(&mut self, command: &str) -> Result<()> {
        let _ = self.pipe.write(command.as_bytes())?;
        Ok(())
    }
}

impl UserData for SoundPadPipe {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("write", |_, this: &mut SoundPadPipe, command: String| {
            this.write(&command)
                .unwrap_or_else(|_| println!("Error: soundpad pipe is not available!"));
            Ok(())
        });
    }
}

pub struct RconWrapper {
    pub conn: Connection,
}

impl RconWrapper {
    pub fn new(_conn: Connection) -> Self {
        Self { conn: _conn }
    }

    pub async fn send_command<S: Into<String>>(&mut self, command: S) -> Result<String> {
        let resul = self.conn.cmd(&command.into()).await?;
        Ok(resul)
    }

    pub async fn say<S: Into<String>>(&mut self, command: S) {
        let string: String = command.into().replace(';', "");
        let _ = self.send_command(format!("say {}", &string)).await;
    }

    pub async fn taunt(&mut self, num: u32) -> Result<String> {
        let resul = self.send_command(&format!("taunt {}", num)).await?;
        Ok(resul)
    }

    pub async fn wait(&mut self, time: u64) {
        task::sleep(Duration::from_millis(time)).await;
    }
}

impl UserData for RconWrapper {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut("taunt", |_, this: &mut RconWrapper, val: u32| async move {
            let _ = this
                .taunt(val)
                .await
                .unwrap_or_else(|_| panic!("Error taunting!"));
            Ok(())
        });

        methods.add_async_method_mut("wait", |_, this: &mut RconWrapper, val: u64| async move {
            this.wait(val).await;
            Ok(())
        });

        methods.add_async_method_mut("say", |_, this: &mut RconWrapper, val: String| async move {
            this.say(val).await;
            Ok(())
        });

        methods.add_async_method_mut(
            "send_command",
            |_, this: &mut RconWrapper, val: String| async move {
                let resul = this
                    .send_command(val)
                    .await
                    .unwrap_or_else(|_| panic!("Error sending command"));
                Ok(resul)
            },
        );
    }
}

pub async fn exec_lua_code(
    conn: &mut RconWrapper,
    pipe: &mut Option<SoundPadPipe>,
    code: &str,
    username: &str,
    username_victim: &str,
) -> mlua::Result<bool> {
    let lua = Lua::new();

    let mut was_killed: bool = false;

    lua.scope(|scope| {
        let globals = lua.globals();
        globals.set("tf2", scope.create_userdata_ref_mut(conn)?)?;

        globals.set("OS", cfg!(target_family))?;

        if pipe.is_some() {
            globals.set(
                "soundpad",
                scope.create_userdata_ref_mut(pipe.as_mut().unwrap())?,
            )?;
        }

        // load the custom lua code and execute it
        lua.load(code)
            .set_name("Custom code and callbacks")
            .exec()
            .unwrap_or_else(|err| panic!("{}", err.to_string()));

        // gets the callback we are going to call hehe
        let on_kill: Function = globals
            .get("OnKillCallback")
            .unwrap_or_else(|err| panic!("{}", err.to_string()));

        // hopefully username is you and username_victim is your, well, victim.
        was_killed = task::block_on(on_kill.call_async::<_, bool>((username, username_victim)))
            .unwrap_or_else(|err| panic!("{}", err.to_string()));

        Ok(())
    })?;

    Ok(was_killed)
}
