use mlua::lua_State;

use crate::add_preload_module;

extern "C-unwind" {
    fn luaopen_lpeglabel(lua: *mut lua_State) -> i32;
}

pub fn register_lpeglabel_module(lua: &mlua::Lua) -> mlua::Result<()> {
    let lpeglabel_loader = unsafe { lua.create_c_function(luaopen_lpeglabel) }?;
    add_preload_module(&lua, "lpeglabel", lpeglabel_loader)?;
    Ok(())
}