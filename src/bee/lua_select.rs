use mlua::prelude::LuaResult;
use mlua::prelude::*;
use super::socket::lua_select::LuaSelect;



fn bee_select_create(_: &Lua, _: ()) -> LuaResult<LuaSelect> {
    Ok(LuaSelect::new())
}

pub fn bee_select(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("create", lua.create_function(bee_select_create)?)?;
    table.set("SELECT_READ", 1)?;
    table.set("SELECT_WRITE", 2)?;
    Ok(table)
}
