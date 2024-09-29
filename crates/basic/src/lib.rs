pub mod bee;
pub mod codestyle;
pub mod lua_seri;
pub mod override_lua;
mod utf8;

#[macro_use]
extern crate lazy_static;
#[allow(unused)]
use crate::codestyle::fake_code_style;
use mlua::{lua_State, prelude::*};
use override_lua::encoder;

extern "C-unwind" {
    fn luaopen_lpeglabel(lua: *mut lua_State) -> i32;

    #[cfg(not(feature = "no_format"))]
    fn luaopen_code_format(lua: *mut lua_State) -> i32;
}

pub fn lua_preload(lua: &Lua) -> LuaResult<()> {
    // lpeglabel
    let lpeglabel_loader = unsafe { lua.create_c_function(luaopen_lpeglabel) }.unwrap();
    add_preload_module(&lua, "lpeglabel", lpeglabel_loader)?;
    // code_format
    #[cfg(feature = "no_format")]
    {
        let code_format_loader = lua.create_function(|lua: &Lua, ()| Ok(fake_code_style(lua)))?;
        add_preload_module(&lua, "code_format", code_format_loader)?;
    }
    #[cfg(not(feature = "no_format"))]
    {
        let code_format_loader = unsafe { lua.create_c_function(luaopen_code_format) }?;
        add_preload_module(&lua, "code_format", code_format_loader)?;
    }

    utf8::register_lua_utf8(lua)?;
    let encoder_loader = lua.create_function(|lua: &Lua, ()| Ok(encoder::lua_encoder_loader(lua)))?;
    add_preload_module(&lua, "encoder", encoder_loader)?;

    // bee.platform
    let bee_platform_loader =
        lua.create_function(|lua: &Lua, ()| Ok(bee::lua_platform::bee_platform(lua)))?;
    add_preload_module(&lua, "bee.platform", bee_platform_loader)?;
    // bee.filesystem
    let bee_filesystem_loader =
        lua.create_function(|lua: &Lua, ()| Ok(bee::lua_filesystem::bee_filesystem(lua)))?;
    add_preload_module(&lua, "bee.filesystem", bee_filesystem_loader)?;
    // bee.thread
    let bee_thread_loader =
        lua.create_function(|lua: &Lua, ()| Ok(bee::lua_thread::bee_thread(lua)))?;
    add_preload_module(&lua, "bee.thread", bee_thread_loader)?;
    // bee.time
    let bee_time_loader = lua.create_function(|lua: &Lua, ()| Ok(bee::lua_time::bee_time(lua)))?;
    add_preload_module(&lua, "bee.time", bee_time_loader)?;
    // bee.windows
    let bee_windows_loader =
        lua.create_function(|lua: &Lua, ()| Ok(bee::lua_windows::bee_windows(lua)))?;
    add_preload_module(&lua, "bee.windows", bee_windows_loader)?;
    // bee.subprocess
    let bee_subprocess_loader =
        lua.create_function(|lua: &Lua, ()| Ok(bee::lua_subprocess::bee_subprocess(lua)))?;
    add_preload_module(&lua, "bee.subprocess", bee_subprocess_loader)?;
    // bee.socket
    let bee_socket_loader =
        lua.create_function(|lua: &Lua, ()| Ok(bee::lua_socket::bee_socket(lua)))?;
    add_preload_module(&lua, "bee.socket", bee_socket_loader)?;
    // bee.select
    let bee_select_loader =
        lua.create_function(|lua: &Lua, ()| Ok(bee::lua_select::bee_select(lua)))?;
    add_preload_module(&lua, "bee.select", bee_select_loader)?;
    // bee.filewatch
    let bee_filewatch_loader =
        lua.create_function(|lua: &Lua, ()| Ok(bee::lua_filewatch::bee_filewatch(lua)))?;
    add_preload_module(&lua, "bee.filewatch", bee_filewatch_loader)?;

    // lua_seri
    lua_seri::register_lua_seri(&lua)?;

    // override

    add_package_path(
        &lua,
        vec![
            "resources/?.lua;resources/?/init.lua;",
            "resources/override_script/?.lua;resources/override_script/?/init.lua;",
            "resources/script/?.lua;resources/script/?/init.lua"
        ],
    )?;

    Ok(())
}

fn add_preload_module(lua: &Lua, name: &str, loader: LuaFunction) -> LuaResult<()> {
    let preload = lua
        .globals()
        .get::<LuaTable>("package")?
        .get::<LuaTable>("preload")?;
    preload.set(name, loader)?;
    Ok(())
}

fn add_package_path(lua: &Lua, paths: Vec<&str>) -> LuaResult<()> {
    let package = lua.globals().get::<LuaTable>("package")?;
    let package_path = package.get::<String>("path")?;
    let path = paths.join(";");

    package.set("path", format!("{};{}", path, package_path))?;
    Ok(())
}