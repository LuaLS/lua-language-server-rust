use mlua::ffi::lua_Integer;
use mlua::{ffi, lua_State, prelude::*, Lua};
use std::ffi::c_void;
use std::os::raw::c_int;

extern "C-unwind" {
    fn seri_unpackptr(lua: *mut lua_State, buffer: *mut c_void) -> i32;

    fn seri_pack(lua: *mut lua_State, from: i32, sz: *mut c_int) -> *mut c_void;
}

unsafe extern "C-unwind" fn lua_seri_pack(lua_state: *mut lua_State) -> i32 {
    let top = ffi::lua_gettop(lua_state);
    if top < 1 {
        return 0;
    }

    let buffer_id = seri_pack(lua_state, 0, std::ptr::null_mut());
    
    ffi::lua_pushinteger(lua_state, buffer_id as lua_Integer);
    1
}

unsafe extern "C-unwind" fn lua_seri_unpack(lua_state: *mut lua_State) -> i32 {
    let top = ffi::lua_gettop(lua_state);
    if top < 1 {
        return 0;
    }

    let buffer_id = ffi::lua_tointeger(lua_state, 1);
    seri_unpackptr(lua_state, buffer_id as *mut c_void)
}

pub fn register_lua_seri(lua: &Lua) -> LuaResult<()> {
    let lua_seri_pack = unsafe { lua.create_c_function(lua_seri_pack).unwrap() };
    lua.globals().set("lua_seri_pack", lua_seri_pack)?;

    let lua_seri_unpack = unsafe { lua.create_c_function(lua_seri_unpack).unwrap() };
    lua.globals().set("lua_seri_unpack", lua_seri_unpack)?;

    Ok(())
}
