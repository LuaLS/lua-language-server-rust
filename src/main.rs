mod bee;
mod codestyle;
mod lua_preload;
mod lua_seri;
mod override_lua;

#[macro_use]
extern crate lazy_static;
use mlua::prelude::*;
use std::{env, path};

#[tokio::main(flavor = "current_thread")]
async fn main() -> LuaResult<()> {
    let lua = unsafe { Lua::unsafe_new() };
    lua_preload::lua_preload(&lua)?;

    #[cfg(not(debug_assertions))]
    {
        let exe_path = env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();
        std::env::set_current_dir(exe_dir)?;
    }

    build_args(&lua);
    let main = lua.load(path::Path::new("resources/main.lua"));
    main.call_async(()).await?;
    Ok(())
}

fn build_args(lua: &Lua) {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let table = lua.create_table().unwrap();
    for (i, arg) in args.iter().enumerate() {
        table.set(i + 1, arg.clone()).unwrap();
    }
    let exe_path = env::current_exe().unwrap();
    table.set(-1, exe_path.to_str().unwrap()).unwrap();
    lua.globals().set("arg", table).unwrap();
}

#[cfg(test)]
mod tests {
    use tokio::runtime::Builder;

    use super::*;
    #[test]
    fn test_main() {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(async move {
            let lua = unsafe { Lua::unsafe_new() };
            if let Err(e) = lua_preload::lua_preload(&lua) {
                eprintln!("Error during lua_preload: {:?}", e);
                return;
            }
            let main = lua.load(path::Path::new("resources/test.lua"));
            main.call_async::<()>(()).await.unwrap();
        });
    }
}
