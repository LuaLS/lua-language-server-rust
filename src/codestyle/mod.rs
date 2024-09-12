use mlua::Lua;

#[allow(unused)]
fn not_implement(_: &Lua, _: mlua::MultiValue) -> mlua::Result<(bool, String)> {
    Ok((false, "not implement".to_string()))
}

#[allow(unused)]
pub fn fake_code_style(lua: &Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;
    table.set("format", lua.create_function(not_implement)?)?;
    table.set("range_format", lua.create_function(not_implement)?)?;
    table.set("type_format", lua.create_function(not_implement)?)?;
    table.set("update_config", lua.create_function(not_implement)?)?;
    table.set("diagnose_file", lua.create_function(not_implement)?)?;
    table.set("set_default_config", lua.create_function(not_implement)?)?;
    table.set("spell_load_dictionary_from_path", lua.create_function(not_implement)?)?;
    table.set("spell_load_dictionary_from_buffer", lua.create_function(not_implement)?)?;
    table.set("spell_analysis", lua.create_function(not_implement)?)?;
    table.set("spell_suggest", lua.create_function(not_implement)?)?;
    table.set("set_nonstandard_symbol", lua.create_function(not_implement)?)?;
    table.set("set_clike_comments_symbol", lua.create_function(not_implement)?)?;
    table.set("name_style_analysis", lua.create_function(not_implement)?)?;
    table.set("update_name_style_config", lua.create_function(not_implement)?)?;
    Ok(table)
}
