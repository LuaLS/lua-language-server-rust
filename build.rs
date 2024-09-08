fn main() {
    std::env::set_var("CC_LOG", "1");

    build_lua();
    build_lua_seri();
    build_lpeglabel();
    cfg!(windows).then(|| build_setfilemode());
}

fn build_lua() {
    cc::Build::new()
        .include("3rd/lua")
        .files(
            std::fs::read_dir("3rd/lua")
                .unwrap()
                .filter_map(|entry| {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.extension()?.to_str()? == "c" {
                        Some(path)
                    } else {
                        None
                    }
                }),
        )
        .compile("lua");
}

fn build_lua_seri() {
    cc::Build::new()
        .include("3rd/lua-seri")
        .include("3rd/lua")
        .files(
            std::fs::read_dir("3rd/lua-seri")
                .unwrap()
                .filter_map(|entry| {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.extension()?.to_str()? == "c" {
                        Some(path)
                    } else {
                        None
                    }
                }),
        )
        .compile("lua-seri");
}

fn build_lpeglabel() {
    cc::Build::new()
        .include("3rd/lpeglabel")
        .include("3rd/lua")
        .files(
            std::fs::read_dir("3rd/lpeglabel")
                .unwrap()
                .filter_map(|entry| {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.extension()?.to_str()? == "c" {
                        Some(path)
                    } else {
                        None
                    }
                }),
        )
        .compile("lpeglabel");
}

fn build_setfilemode() {
    cc::Build::new()
        .include("3rd/setfilemode")
        .files(
            std::fs::read_dir("3rd/setfilemode")
                .unwrap()
                .filter_map(|entry| {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.extension()?.to_str()? == "c" {
                        Some(path)
                    } else {
                        None
                    }
                }),
        )
        .compile("setfilemode");
}