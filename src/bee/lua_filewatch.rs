use mlua::prelude::LuaResult;
use mlua::prelude::*;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
// use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

use super::lua_filesystem::LuaFilePath;
// use std::time::Duration;
// use tokio::task;

struct LuaFileWatch {
    watcher: Option<RecommendedWatcher>,
    receiver: Option<Receiver<Event>>,
    recursive: Option<bool>,
    filter: Option<mlua::Function>,
}

impl LuaFileWatch {
    fn new() -> LuaFileWatch {
        LuaFileWatch {
            watcher: None,
            receiver: None,
            recursive: None,
            filter: None,
        }
    }

    fn set_recursive(&mut self, recursive: bool) -> LuaResult<()> {
        self.recursive = Some(recursive);
        Ok(())
    }

    fn set_follow_symlinks(&mut self, _: bool) -> LuaResult<()> {
        Ok(())
    }

    fn set_filter(&mut self, filter: mlua::Function) -> LuaResult<()> {
        self.filter = Some(filter);
        Ok(())
    }

    fn select(&mut self, lua: &Lua) -> LuaResult<(mlua::Value, mlua::Value)> {
        if let Some(rx) = &self.receiver {
            if let Ok(event) = rx.try_recv() {
                let path = event.paths[0].to_str().unwrap().to_string();
                if let Some(filter) = &self.filter {
                    if let Ok(result) = filter.call::<bool>(path.clone()) {
                        if !result {
                            return Ok((mlua::Nil, mlua::Nil));
                        }
                    }
                }
                let kind = match event.kind {
                    notify::EventKind::Create(_) => "create",
                    notify::EventKind::Modify(_) => "modify",
                    notify::EventKind::Remove(_) => "remove",
                    _ => "unknown",
                };

                return Ok((kind.into_lua(lua).unwrap(), path.into_lua(lua).unwrap()));
            }
        }

        Ok((mlua::Nil, mlua::Nil))
    }

    fn add(&mut self, path: LuaFilePath) -> LuaResult<()> {
        let (tx, rx) = channel();
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                match res {
                    Ok(event) => {
                        tx.send(event).unwrap();
                    }
                    _ => {}
                };
            },
            Config::default(),
        )
        .unwrap();
        let recursive = if self.recursive.unwrap_or(false) {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };
        watcher.watch(&path.to_path(), recursive).unwrap();
        self.watcher = Some(watcher);
        self.receiver = Some(rx);
        Ok(())
    }
}

impl LuaUserData for LuaFileWatch {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_recursive", |_, this, recursive: bool| {
            this.set_recursive(recursive)
        });
        methods.add_method_mut("set_follow_symlinks", |_, this, follow_symlinks: bool| {
            this.set_follow_symlinks(follow_symlinks)
        });
        methods.add_method_mut("set_filter", |_, this, filter: mlua::Function| {
            this.set_filter(filter)
        });
        methods.add_method_mut("select", |lua, this, ()| this.select(lua));
        methods.add_method_mut("add", |_, this, path: LuaFilePath| this.add(path));
    }
}

fn bee_filewatch_create(_: &Lua, _: ()) -> LuaResult<LuaFileWatch> {
    Ok(LuaFileWatch::new())
}

pub fn bee_filewatch(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("create", lua.create_function(bee_filewatch_create)?)?;
    Ok(table)
}
