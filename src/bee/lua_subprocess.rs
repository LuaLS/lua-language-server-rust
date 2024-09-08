use mlua::prelude::LuaResult;
use mlua::prelude::*;
use std::process;

fn bee_subprocess_spawn(_: &Lua, _: ()) -> LuaResult<()> {
    Ok(())
}

fn bee_subprocess_get_id(_: &Lua, _: ()) -> LuaResult<u64> {
    Ok(process::id() as u64)
}

pub fn bee_subprocess(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("spawn", lua.create_function(bee_subprocess_spawn)?)?;
    table.set("get_id", lua.create_function(bee_subprocess_get_id)?)?;
    Ok(table)
}