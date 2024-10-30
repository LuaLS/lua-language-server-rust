mod lua_parser;
mod lua_syntax_tree;
mod lua_node;

use mlua::{prelude::*, Lua};

use crate::add_preload_module;


pub fn register_parser_module(lua: &Lua) -> LuaResult<()> {
    // lua.parser
    let lua_parser_loader =
        lua.create_function(|lua: &Lua, ()| Ok(lua_parser::lua_parser(lua)))?;
    add_preload_module(&lua, "lua.parser", lua_parser_loader)?;
    Ok(())
}
