mod lua_filesystem;
mod lua_filewatch;
mod lua_platform;
mod lua_select;
mod lua_socket;
mod lua_subprocess;
mod lua_thread;
mod lua_time;
mod lua_windows;
mod socket;
use mlua::prelude::*;

use crate::add_preload_module;

pub fn register_bee_modules(lua: &Lua) -> LuaResult<()> {
    // bee.platform
    let bee_platform_loader =
        lua.create_function(|lua: &Lua, ()| Ok(lua_platform::bee_platform(lua)))?;
    add_preload_module(&lua, "bee.platform", bee_platform_loader)?;
    // bee.filesystem
    let bee_filesystem_loader =
        lua.create_function(|lua: &Lua, ()| Ok(lua_filesystem::bee_filesystem(lua)))?;
    add_preload_module(&lua, "bee.filesystem", bee_filesystem_loader)?;
    // bee.thread
    let bee_thread_loader = lua.create_function(|lua: &Lua, ()| Ok(lua_thread::bee_thread(lua)))?;
    add_preload_module(&lua, "bee.thread", bee_thread_loader)?;
    // bee.time
    let bee_time_loader = lua.create_function(|lua: &Lua, ()| Ok(lua_time::bee_time(lua)))?;
    add_preload_module(&lua, "bee.time", bee_time_loader)?;
    // bee.windows
    let bee_windows_loader =
        lua.create_function(|lua: &Lua, ()| Ok(lua_windows::bee_windows(lua)))?;
    add_preload_module(&lua, "bee.windows", bee_windows_loader)?;
    // bee.subprocess
    let bee_subprocess_loader =
        lua.create_function(|lua: &Lua, ()| Ok(lua_subprocess::bee_subprocess(lua)))?;
    add_preload_module(&lua, "bee.subprocess", bee_subprocess_loader)?;
    // bee.socket
    let bee_socket_loader = lua.create_function(|lua: &Lua, ()| Ok(lua_socket::bee_socket(lua)))?;
    add_preload_module(&lua, "bee.socket", bee_socket_loader)?;
    // bee.select
    let bee_select_loader = lua.create_function(|lua: &Lua, ()| Ok(lua_select::bee_select(lua)))?;
    add_preload_module(&lua, "bee.select", bee_select_loader)?;
    // bee.filewatch
    let bee_filewatch_loader =
        lua.create_function(|lua: &Lua, ()| Ok(lua_filewatch::bee_filewatch(lua)))?;
    add_preload_module(&lua, "bee.filewatch", bee_filewatch_loader)?;

    Ok(())
}
