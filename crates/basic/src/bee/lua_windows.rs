use std::borrow::Cow;
use std::io;

use encoding_rs::WINDOWS_1252;
use mlua::prelude::LuaResult;
use mlua::prelude::*;

#[cfg(windows)]
mod windows {
    use std::ffi::c_void;

    extern "C" {
        fn setfilebinary() -> c_void;
    }

    pub fn set_stdio_to_binary() -> std::io::Result<()> {
        unsafe {
            setfilebinary();
        }
        Ok(())
    }
}

fn bee_windows_u2a(_: &Lua, text: String) -> LuaResult<String> {
    // Convert the input Unicode string to bytes
    let (encoded, _, had_errors) = WINDOWS_1252.encode(&text);

    // Check if there were any encoding errors
    if had_errors {
        return Err(mlua::Error::RuntimeError("Encoding error".to_string()));
    }

    // Convert the encoded bytes back to a String
    match String::from_utf8(encoded.to_vec()) {
        Ok(ascii_string) => Ok(ascii_string),
        Err(_) => Err(mlua::Error::RuntimeError(
            "Conversion to ASCII failed".to_string(),
        )),
    }
}

fn bee_windows_a2u(_: &Lua, text: String) -> LuaResult<String> {
    // Convert the input ASCII string to bytes
    let (decoded, _, had_errors) = WINDOWS_1252.decode(text.as_bytes());

    // Check if there were any decoding errors
    if had_errors {
        return Err(mlua::Error::RuntimeError("Decoding error".to_string()));
    }

    // Convert the decoded bytes back to a String
    match decoded {
        Cow::Borrowed(unicode_string) => Ok(unicode_string.to_string()),
        Cow::Owned(unicode_string) => Ok(unicode_string),
    }
}

fn set_stdio_to_binary() -> io::Result<()> {
    #[cfg(windows)]
    {
        windows::set_stdio_to_binary()?;
    }
    Ok(())
}

fn bee_windows_filemode(_: &Lua, args: mlua::MultiValue) -> LuaResult<()> {
    if args.len() != 2 {
        return Err(mlua::Error::RuntimeError(
            "Invalid number of arguments".to_string(),
        ));
    }

    if cfg!(target_os = "windows") {
        set_stdio_to_binary()?;
    }
    Ok(())
}

pub fn bee_windows(lua: &Lua) -> LuaResult<LuaTable> {
    let windows = lua.create_table()?;
    windows.set("u2a", lua.create_function(bee_windows_u2a)?)?;
    windows.set("a2u", lua.create_function(bee_windows_a2u)?)?;
    windows.set("filemode", lua.create_function(bee_windows_filemode)?)?;
    Ok(windows)
}
