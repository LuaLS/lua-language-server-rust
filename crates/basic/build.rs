fn main() {
    std::env::set_var("CC_LOG", "1");
    std::env::set_current_dir("../..").unwrap();
    // build_lua();
    build_lua_seri();
    build_lpeglabel();
    cfg!(windows).then(|| build_setfilemode());
    cfg!(not(feature = "no_format")).then(|| build_emmyluacodestyle());
}

#[allow(unused)]
fn build_lua() {
    cc::Build::new()
        .include("3rd/lua")
        .files(std::fs::read_dir("3rd/lua").unwrap().filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension()?.to_str()? == "c" {
                Some(path)
            } else {
                None
            }
        }))
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

fn build_emmyluacodestyle() {
    let mut builder = cc::Build::new();
    builder.cpp(true);
    builder
        .include("3rd/EmmyLuaCodeStyle/Util/include")
        .include("3rd/EmmyLuaCodeStyle/CodeFormatCore/include")
        .include("3rd/EmmyLuaCodeStyle/LuaParser/include")
        .include("3rd/EmmyLuaCodeStyle/3rd/wildcards/include")
        .include("3rd/lua");

    let file_patterns = vec![
        "3rd/EmmyLuaCodeStyle/CodeFormatLib/src/*.cpp",
        "3rd/EmmyLuaCodeStyle/LuaParser/src/**/*.cpp",
        "3rd/EmmyLuaCodeStyle/Util/src/StringUtil.cpp",
        "3rd/EmmyLuaCodeStyle/Util/src/Utf8.cpp",
        "3rd/EmmyLuaCodeStyle/Util/src/SymSpell/*.cpp",
        "3rd/EmmyLuaCodeStyle/Util/src/InfoTree/*.cpp",
        "3rd/EmmyLuaCodeStyle/CodeFormatCore/src/**/*.cpp",
    ];

    for pattern in file_patterns {
        if pattern.contains("*") {
            builder.files(glob::glob(pattern).unwrap().filter_map(|path| path.ok()));
        } else {
            builder.file(pattern);
        }
    }

    if cfg!(windows) {
        builder.flag("/utf-8");
        builder.flag("/std:c++17");
    } else {
        builder.flag("-std=c++17");
    }

    builder.compile("EmmyLuaCodeStyle");
}
