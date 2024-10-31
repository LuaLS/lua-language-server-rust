mod bee;
mod codestyle;
mod lpeglabel;
mod lua_seri;
mod override_lua;
mod parser;

#[macro_use]
extern crate lazy_static;
use mlua::prelude::*;

pub fn lua_preload(lua: &Lua) -> LuaResult<()> {
    // lpeglabel
    lpeglabel::register_lpeglabel_module(lua)?;
    // bee modules
    bee::register_bee_modules(lua)?;
    // rust override lua modules
    override_lua::register_override_modules(lua)?;
    // lua_seri
    lua_seri::register_lua_seri(&lua)?;
    // codestyle
    codestyle::register_code_format_module(lua)?;

    parser::register_parser_module(lua)?;
    // Get current environment path
    let current_path = std::env::current_dir()?;
    let base_require_path = current_path.join("?.lua");
    let base_require_path_init = current_path.join("?/init.lua");
    let require_path = current_path.join("script/?.lua");
    let require_path_init = current_path.join("script/?/init.lua");
    add_package_path(
        &lua,
        vec![
            base_require_path.to_str().unwrap(),
            base_require_path_init.to_str().unwrap(),
            require_path.to_str().unwrap(),
            require_path_init.to_str().unwrap(),
        ],
    )?;

    Ok(())
}

pub(crate) fn add_preload_module(lua: &Lua, name: &str, loader: LuaFunction) -> LuaResult<()> {
    let preload = lua
        .globals()
        .get::<LuaTable>("package")?
        .get::<LuaTable>("preload")?;
    preload.set(name, loader)?;
    Ok(())
}

pub(crate) fn add_package_path(lua: &Lua, paths: Vec<&str>) -> LuaResult<()> {
    let package = lua.globals().get::<LuaTable>("package")?;
    let package_path = package.get::<String>("path")?;
    let path = paths.join(";");

    package.set("path", format!("{};{}", path, package_path))?;
    Ok(())
}
