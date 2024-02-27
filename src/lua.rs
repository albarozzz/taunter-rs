use mlua::{FromLua, Function, Lua, MetaMethod, UserData, UserDataMethods, Value, Variadic};

use crate::helper::send_command;
use async_std::task;
use std::time::Duration;

pub enum Command {
    Message(String),
    RawCommand(String),
}

pub async fn exec_lua_code(
    conn: &mut rcon::Connection,
    code: &str,
    username: &str,
    username_victim: &str,
) -> mlua::Result<bool> {
    let lua = Lua::new();
    let globals = lua.globals();
    // TODO: create a function wrapper(? to allow the lua code to send commands directly to tf2 (impossible??)
    // globals.set("send_command", lua.create_async_function( |_ctx: &Lua, msg: String| async move { send_command_lua(&mut mtd, &msg).await; mlua::Result::Ok(()) })?)?;

    // load the custom lua code and execute it
    lua.load(code)
        .set_name("Custom code and callbacks")
        .exec()?;

    // gets the callback we are going to call hehe
    let on_kill: Function = globals.get("OnKillCallback")?;

    // hopefully username is you and username_victim is your, well, victim.
    let table = on_kill
        .call_async::<_, mlua::Table>((username, username_victim))
        .await?;

    let mut has_done_smt: bool = false;
    for pair in table.pairs::<String, String>() {
        let (key, value) = pair?;
        match key.as_str() {
            "MESSAGE" => {
                let _ = send_command(conn, &format!("say {}", &value)).await;
                has_done_smt = true; // acknowledge the response
            }
            "RAW_COMMAND" => {
                let _ = send_command(conn, &value).await;
                has_done_smt = true; // acknowledge the response
            }
            "WAIT" => {
                task::sleep(Duration::from_millis(value.parse::<u64>().unwrap_or(0))).await;
                has_done_smt = true; // acknowledge the response
            }
            _ => println!("Ignoring unknown command from lua"),
        };
        task::sleep(Duration::from_millis(100)).await;
    }

    Ok(has_done_smt)
}
