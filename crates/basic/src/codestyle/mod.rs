#[allow(unused)]
use mlua::{lua_State, prelude::*};

use crate::add_preload_module;

extern "C-unwind" {
    #[cfg(not(feature = "no_format"))]
    fn luaopen_code_format(lua: *mut lua_State) -> i32;
}

#[allow(unused)]
fn not_implement(_: &Lua, _: mlua::MultiValue) -> LuaResult<(bool, String)> {
    Ok((false, "not implement".to_string()))
}

#[allow(unused)]
fn fake_code_style(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("format", lua.create_function(not_implement)?)?;
    table.set("range_format", lua.create_function(not_implement)?)?;
    table.set("type_format", lua.create_function(not_implement)?)?;
    table.set("update_config", lua.create_function(not_implement)?)?;
    table.set("diagnose_file", lua.create_function(not_implement)?)?;
    table.set("set_default_config", lua.create_function(not_implement)?)?;
    table.set(
        "spell_load_dictionary_from_path",
        lua.create_function(not_implement)?,
    )?;
    table.set(
        "spell_load_dictionary_from_buffer",
        lua.create_function(not_implement)?,
    )?;
    table.set("spell_analysis", lua.create_function(not_implement)?)?;
    table.set("spell_suggest", lua.create_function(not_implement)?)?;
    table.set(
        "set_nonstandard_symbol",
        lua.create_function(not_implement)?,
    )?;
    table.set(
        "set_clike_comments_symbol",
        lua.create_function(not_implement)?,
    )?;
    table.set("name_style_analysis", lua.create_function(not_implement)?)?;
    table.set(
        "update_name_style_config",
        lua.create_function(not_implement)?,
    )?;
    Ok(table)
}

pub fn register_code_format_module(lua: &Lua) -> LuaResult<()> {
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

    Ok(())
}
