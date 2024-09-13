use mlua::prelude::LuaResult;
use mlua::prelude::*;
use std::borrow::BorrowMut;
use std::io::{self};
use std::process::{id as process_id, Child, Command, Stdio};

struct LuaSubprocess {
    child: Option<Child>,
}

impl LuaSubprocess {
    fn new() -> Self {
        LuaSubprocess { child: None }
    }

    fn start(&mut self, command: &str, args: &[String]) -> io::Result<()> {
        let child = Command::new(command)
            .arg(args.join(" "))
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        self.child = Some(child);
        Ok(())
    }

    fn wait(&mut self) {
        if let Some(child) = self.child.borrow_mut() {
            child.wait().unwrap();
        }
    }
    fn get_id(&self) -> u64 {
        if let Some(child) = &self.child {
            child.id() as u64
        } else {
            0
        }
    }

    fn is_running(&self) -> bool {
        self.child.is_some()
    }
}

impl LuaUserData for LuaSubprocess {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("wait", |_, this, _: ()| {
            this.wait();
            Ok(())
        });

        methods.add_method("get_id", |_, this, _: ()| {
            Ok(this.get_id())
        });

        methods.add_method("is_running", |_, this, _: ()| {
            Ok(this.is_running())
        });
    }
}


fn bee_subprocess_spawn(_: &Lua, args: mlua::Table) -> LuaResult<LuaSubprocess> {
    let mut exe: String = String::new();
    let mut args_string: Vec<String> = vec![];
    for pair in args.pairs::<i32, String>() {
        if let Ok((i, arg)) = pair {
            if i == 1 {
                exe = arg;
                continue;
            }

            args_string.push(arg);
        }
    }

    let mut subprocess = LuaSubprocess::new();
    subprocess.start(&exe, &args_string).unwrap();

    Ok(subprocess)
}

fn bee_subprocess_get_id(_: &Lua, _: ()) -> LuaResult<u64> {
    Ok(process_id() as u64)
}

pub fn bee_subprocess(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("spawn", lua.create_function(bee_subprocess_spawn)?)?;
    table.set("get_id", lua.create_function(bee_subprocess_get_id)?)?;
    Ok(table)
}
