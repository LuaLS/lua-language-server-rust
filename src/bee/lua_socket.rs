use super::socket::lua_socket::{LuaSocket, SocketType};
use super::socket::lua_socket_pool::SOCKET_POOL;
use mlua::prelude::LuaResult;
use mlua::prelude::*;

async fn bee_socket_create(_: Lua, protocol: String) -> LuaResult<LuaSocket> {
    let mut socket_pool = SOCKET_POOL.lock().await;
    let socket = match protocol.as_str() {
        "tcp" => socket_pool.create_socket(SocketType::Tcp).unwrap(),
        "unix" => socket_pool.create_socket(SocketType::Unix).unwrap(),
        _ => return Err(LuaError::external("Invalid protocol")),
    };

    Ok(socket)
}

pub fn bee_socket(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("create", lua.create_async_function(bee_socket_create)?)?;
    Ok(table)
}
