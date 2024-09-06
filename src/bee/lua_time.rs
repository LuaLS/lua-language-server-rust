use mlua::prelude::LuaResult;
use mlua::prelude::*;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn bee_time_time(_: &Lua, _: ()) -> LuaResult<i64> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get system time")
        .as_millis() as i64;
    Ok(timestamp)
}

fn bee_time_monotonic(_: &Lua, _: ()) -> LuaResult<i64> {
    let duration = Instant::now().elapsed();
    let timestamp = duration.as_millis() as i64;
    Ok(timestamp)
}

pub fn bee_time(lua: &Lua) -> LuaResult<LuaTable> {
    let time = lua.create_table()?;
    time.set("time", lua.create_function(bee_time_time)?)?;
    time.set("monotonic", lua.create_function(bee_time_monotonic)?)?;
    Ok(time)
}
