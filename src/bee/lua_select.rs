use mlua::prelude::LuaResult;
use mlua::prelude::*;

struct LuaSelect {
}

impl LuaSelect {
    fn new() -> LuaSelect {
        LuaSelect {}
    }

    fn wait() -> LuaResult<()> {
        // Implementation for wait function
        Ok(())
    }

    fn close() -> LuaResult<()> {
        // Implementation for close function
        Ok(())
    }

    fn event_add() -> LuaResult<()> {
        // Implementation for event_add function
        Ok(())
    }

    fn event_mod() -> LuaResult<()> {
        // Implementation for event_mod function
        Ok(())
    }

    fn event_del() -> LuaResult<()> {
        // Implementation for event_del function
        Ok(())
    }
}


fn bee_select_create(_: &Lua, _: ()) -> LuaResult<()> {
    Ok(())
}

pub fn bee_select(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("create", lua.create_function(bee_select_create)?)?;
    table.set("SELECT_READ", 1)?;
    table.set("SELECT_WRITE", 2)?;
    Ok(table)
}