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
