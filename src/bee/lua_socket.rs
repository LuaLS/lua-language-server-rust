use mlua::prelude::LuaResult;
use mlua::prelude::*;

struct LuaSocekt {
    protocol: String,
}

impl LuaSocekt {
    fn new(protocol: String) -> LuaSocekt {
        LuaSocekt { protocol }
    }
}

impl LuaUserData for LuaSocekt {}


fn bee_socket_create(_: &Lua, protocol: String) -> LuaResult<LuaSocekt> {
    Ok(LuaSocekt::new(protocol))
}

pub fn bee_socket(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("create", lua.create_function(bee_socket_create)?)?;
    Ok(table)
}