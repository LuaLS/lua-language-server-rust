use emmylua_parser::{LuaParser, ParserConfig};
use mlua::{prelude::*, Lua};

use super::lua_syntax_tree::LuaSyntaxTree;

fn parse(_: &Lua, text: String) -> LuaResult<LuaSyntaxTree> {
    let tree = LuaParser::parse(&text, ParserConfig::default());

    Ok(LuaSyntaxTree::new(tree))
}

pub fn lua_parser(lua: &Lua) -> LuaResult<LuaTable> {
    let parser = lua.create_table()?;
    parser.set("parse", lua.create_function(parse)?)?;
    Ok(parser)
}
