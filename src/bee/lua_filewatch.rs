use mlua::prelude::LuaResult;
use mlua::prelude::*;
// use notify::{Event, RecursiveMode, Watcher};
// use std::path::PathBuf;
// use std::sync::mpsc::{channel, Receiver};
// use std::time::Duration;
// use tokio::task;

struct LuaFileWatch {
}

impl LuaFileWatch {
    fn new() -> LuaFileWatch {
        LuaFileWatch {
        }
    }

    fn set_recursive(&mut self, recursive: bool) -> LuaResult<()> {
        Ok(())
    }

    fn set_follow_symlinks(&mut self, follow_symlinks: bool) -> LuaResult<()> {
        Ok(())
    }

    fn set_filter(&mut self, filter: mlua::Function) -> LuaResult<()> {
        Ok(())
    }

    fn select(&mut self) -> LuaResult<(String, String)> {
        Ok(("".to_string(), "".to_string()))
    }

    fn add(&mut self, path: String) -> LuaResult<()> {
        Ok(())
    }
}

impl LuaUserData for LuaFileWatch {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_recursive", |_, this, recursive: bool| this.set_recursive(recursive));
        methods.add_method_mut("set_follow_symlinks", |_, this, follow_symlinks: bool| this.set_follow_symlinks(follow_symlinks));
        methods.add_method_mut("set_filter", |_, this, filter: mlua::Function| this.set_filter(filter));
        methods.add_method_mut("select", |_, this, ()| this.select());
        methods.add_method_mut("add", |_, this, path: String| this.add(path));
    }

}

fn bee_filewatch_create(_: &Lua, _: ()) -> LuaResult<LuaFileWatch> {
    Ok(LuaFileWatch {})
}

pub fn bee_filewatch(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("create", lua.create_function(bee_filewatch_create)?)?;
    Ok(table)
}
