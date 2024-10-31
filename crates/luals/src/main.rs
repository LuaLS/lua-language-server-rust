use mlua::prelude::*;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> LuaResult<()> {
    dynamic_set_root();

    let lua = unsafe { Lua::unsafe_new() };
    luals_basic::lua_preload(&lua)?;

    build_args(&lua);
    let current_path = std::env::current_dir()?;
    let main_path = current_path.join("main.lua");
    let main = lua.load(main_path);
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

fn dynamic_set_root() {
    let exe_path = env::current_exe().unwrap();
    let mut current_dir = exe_path.parent().unwrap();

    while !current_dir.join("main.lua").exists() {
        if let Some(parent) = current_dir.parent() {
            current_dir = parent;
        } else {
            break;
        }
    }

    std::env::set_current_dir(current_dir).unwrap();
}

#[cfg(test)]
mod tests {
    use std::path;

    use tokio::runtime::Builder;

    use super::*;
    #[test]
    fn test_main() {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(async move {
            let lua = unsafe { Lua::unsafe_new() };
            if let Err(e) = luals_basic::lua_preload(&lua) {
                eprintln!("Error during lua_preload: {:?}", e);
                return;
            }
            let main = lua.load(path::Path::new("test.lua"));
            main.call_async::<()>(()).await.unwrap();
        });
    }
}
