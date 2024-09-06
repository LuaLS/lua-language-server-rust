use mlua::prelude::*;
use mlua::Lua;

fn os_version(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("major", 0)?;
    table.set("minor", 0)?;
    table.set("revision", 1)?;

    Ok(table)
}

pub fn bee_platform(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    // 设置操作系统名称
    let os_name = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "android") {
        "android"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "netbsd") {
        "netbsd"
    } else if cfg!(target_os = "freebsd") {
        "freebsd"
    } else if cfg!(target_os = "openbsd") {
        "openbsd"
    } else if cfg!(target_os = "ios") {
        "ios"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "unknown"
    };
    exports.set("os", os_name)?;

    // // 设置编译器信息
    // let (compiler, compiler_version) = if cfg!(target_env = "msvc") {
    //     ("msvc", format!("MSVC {}", env!("VC_REVISION")))
    // } else if cfg!(target_env = "gnu") {
    //     ("gcc", format!("GCC {}.{}.{}", env!("CARGO_CFG_GNUC_VERSION_MAJOR"), env!("CARGO_CFG_GNUC_VERSION_MINOR"), env!("CARGO_CFG_GNUC_VERSION_PATCH")))
    // } else if cfg!(target_env = "clang") {
    //     ("clang", format!("Clang {}.{}.{}", env!("CARGO_CFG_CLANG_VERSION_MAJOR"), env!("CARGO_CFG_CLANG_VERSION_MINOR"), env!("CARGO_CFG_CLANG_VERSION_PATCH")))
    // } else {
    //     ("unknown", "unknown".to_string())
    // };
    // exports.set("Compiler", compiler)?;
    // exports.set("CompilerVersion", compiler_version)?;

    // // 设置 C 运行时库信息
    // let (crt, crt_version) = if cfg!(target_env = "msvc") {
    //     ("msvc", format!("MSVC STL {}", env!("CARGO_CFG_MSC_VER")))
    // } else if cfg!(target_env = "gnu") {
    //     ("libstdc++", format!("libstdc++ {}", env!("CARGO_CFG_GLIBCXX_VERSION")))
    // } else if cfg!(target_env = "musl") {
    //     ("musl", format!("musl {}", env!("CARGO_CFG_MUSL_VERSION")))
    // } else {
    //     ("unknown", "unknown".to_string())
    // };
    // exports.set("CRT", crt)?;
    // exports.set("CRTVersion", crt_version)?;

    // 设置架构信息
    let arch = if cfg!(target_arch = "x86") {
        "x86"
    } else if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else if cfg!(target_arch = "riscv64") {
        "riscv64"
    } else if cfg!(target_arch = "wasm32") {
        "wasm32"
    } else if cfg!(target_arch = "wasm64") {
        "wasm64"
    } else {
        "unknown"
    };
    exports.set("Arch", arch)?;

    // 设置调试信息
    let debug = if cfg!(debug_assertions) { true } else { false };
    exports.set("DEBUG", debug)?;

    // 设置操作系统版本信息
    exports.set("os_version", lua.create_function(|lua, ()| os_version(lua))?)?;

    Ok(exports)
}
