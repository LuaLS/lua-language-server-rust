use mlua::{
    lua_State,
    prelude::*,
    Lua,
};

extern "C-unwind" {
    fn luajit_utf8_len(lua: *mut lua_State) -> i32;

    fn luajit_utf8_char(lua: *mut lua_State) -> i32;

    fn luajit_utf8_codepoint(lua: *mut lua_State) -> i32;

    fn luajit_utf8_codes(lua: *mut lua_State) -> i32;

    fn luajit_utf8_offset(lua: *mut lua_State) -> i32;
}

// compact for luajit
pub fn register_lua_utf8(lua: &Lua) -> LuaResult<()> {
    let utf8 = lua.create_table()?;
    let lua_utf8_len = unsafe { lua.create_c_function(luajit_utf8_len)? };
    let lua_utf8_char = unsafe { lua.create_c_function(luajit_utf8_char)? };
    let lua_utf8_offset = unsafe { lua.create_c_function(luajit_utf8_offset)? };
    let lua_utf8_codepoint = unsafe { lua.create_c_function(luajit_utf8_codepoint)? };
    let lua_utf8_codes = unsafe { lua.create_c_function(luajit_utf8_codes)? };
    utf8.set("len", lua_utf8_len)?;
    utf8.set("char", lua_utf8_char)?;
    utf8.set("offset", lua_utf8_offset)?;
    utf8.set("codepoint", lua_utf8_codepoint)?;
    utf8.set("codes", lua_utf8_codes)?;
    lua.globals().set("utf8", utf8)?;
    Ok(())
}
