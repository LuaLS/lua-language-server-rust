use mlua::prelude::LuaResult;
use mlua::prelude::*;
use super::socket::lua_socket::{LuaSocket, SocketType};


fn bee_socket_create(_: &Lua, protocol: String) -> LuaResult<LuaSocket> {
    let socket = match protocol.as_str() {
        "tcp" => LuaSocket::new(SocketType::Tcp),
        "unix" =>  LuaSocket::new(SocketType::Unix),
        _ => return Err(LuaError::external("Invalid protocol")),
    };

    Ok(socket)
}

pub fn bee_socket(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("create", lua.create_function(bee_socket_create)?)?;
    Ok(table)
}