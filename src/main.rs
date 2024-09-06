mod bee;

#[macro_use]
extern crate lazy_static; 

use std::path;

use bee::*;
use mlua::{lua_State, prelude::*};

extern "C-unwind" {
    fn luaopen_lpeglabel(lua: *mut lua_State) -> i32;
}

fn main() -> LuaResult<()> {
    let lua = unsafe { Lua::unsafe_new() };
    // lpeglabel
    let lpeglabel_loader = unsafe { lua.create_c_function(luaopen_lpeglabel) }.unwrap();
    add_preload_module(&lua, "lpeglabel", lpeglabel_loader)?;
    // bee.platform
    let bee_platform_loader = lua.create_function(|lua: &Lua, ()| Ok(bee::lua_platform::bee_platform(lua)))?;
    add_preload_module(&lua, "bee.platform", bee_platform_loader)?;
    // bee.filesystem
    let bee_filesystem_loader = lua.create_function(|lua: &Lua, ()| Ok(bee::lua_filesystem::bee_filesystem(lua)))?;
    add_preload_module(&lua, "bee.filesystem", bee_filesystem_loader)?;
    // bee.thread
    bee::lua_thread::register_lua_channel(&lua);
    let bee_thread_loader = lua.create_function(|lua: &Lua, ()| Ok(bee::lua_thread::bee_thread(lua)))?;
    add_preload_module(&lua, "bee.thread", bee_thread_loader)?;
    // bee.time
    let bee_time_loader = lua.create_function(|lua: &Lua, ()| Ok(bee::lua_time::bee_time(lua)))?;
    add_preload_module(&lua, "bee.time", bee_time_loader)?;
    // bee.windows
    let bee_windows_loader = lua.create_function(|lua: &Lua, ()| Ok(bee::lua_windows::bee_windows(lua)))?;
    add_preload_module(&lua, "bee.windows", bee_windows_loader)?;

    add_package_path(&lua, "resources/?.lua;resources/?/init.lua")?;

    lua.load(path::Path::new("resources/testmain.lua")).exec()?;
    Ok(())
}

fn add_preload_module(lua: &Lua, name: &str, loader: LuaFunction) -> LuaResult<()> {
    let preload = lua
        .globals()
        .get::<_, LuaTable>("package")?
        .get::<_, LuaTable>("preload")?;
    preload.set(name, loader)?;
    Ok(())
}

fn add_package_path(lua: &Lua, path: &str) -> LuaResult<()> {
    let package = lua.globals().get::<_, LuaTable>("package")?;
    let package_path = package.get::<_, String>("path")?;
    package.set("path", format!("{};{}", path, package_path))?;
    Ok(())
}
