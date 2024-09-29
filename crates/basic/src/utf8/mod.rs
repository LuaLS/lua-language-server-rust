use mlua::{prelude::*, Lua, Value}; 
use std::cmp;

fn lua_utf8_len(
    lua: &Lua,
    (s, i, j, lax): (String, Option<i32>, Option<i32>, Option<bool>),
) -> LuaResult<i32> {
    let len = s.len() as i32;

    let start = match i {
        Some(idx) if idx > 0 => cmp::min(idx - 1, len), // start from 1
        Some(idx) if idx < 0 => cmp::max(len + idx, 0),
        _ => 0,
    };

    let end = match j {
        Some(idx) if idx > 0 => cmp::min(idx, len),
        Some(idx) if idx < 0 => cmp::max(len + idx + 1, 0),
        _ => len,
    };

    let substr = &s[start as usize..end as usize];

    Ok(if lax.unwrap_or(false) {
        substr.chars().count() as i32
    } else {
        match substr.chars().count() {
            count => count as i32,
        }
    })
}

// Receives zero or more integers, converts each one to its corresponding UTF-8 byte sequence and returns a string with the concatenation of all these sequences.
fn lua_utf8_char(lua: &Lua, args: mlua::MultiValue) -> LuaResult<String> {
    let mut result = String::new();

    for arg in args {
        match arg {
            Value::Integer(i) => {
                if let Some(c) = std::char::from_u32(i as u32) {
                    result.push(c);
                }
            }
            _ => {}
        }
    }

    Ok(result)
}

// Returns the codepoints (as integers) from all characters in `s` that start between byte position `i` and `j` (both included).
fn lua_utf8_codepoint(
    lua: &Lua,
    (s, i, j, lax): (String, Option<i32>, Option<i32>, Option<bool>),
) -> LuaResult<mlua::MultiValue> {
    let len = s.len() as i32;

    let start = match i {
        Some(idx) if idx > 0 => cmp::min(idx - 1, len),
        Some(idx) if idx < 0 => cmp::max(len + idx, 0),
        _ => 0,
    };

    let end = match j {
        Some(idx) if idx > 0 => cmp::min(idx, len),
        Some(idx) if idx < 0 => cmp::max(len + idx + 1, 0),
        _ => len,
    };

    let substr = &s[start as usize..end as usize];
    let mut result = mlua::MultiValue::new();

    for c in substr.chars() {
        if lax.unwrap_or(false) {
            if let Some(codepoint) = c.to_digit(10) {
                result.push_back(Value::Integer(codepoint as i64));
            }
        } else {
            result.push_back(Value::Integer(c as u32 as i64));
        }
    }

    Ok(result)
}

// Returns the position (in bytes) where the encoding of the `n`-th character of `s` (counting from position `i`) starts.
fn lua_utf8_offset(lua: &Lua, (s, n, i): (String, i32, Option<i32>)) -> LuaResult<i32> {
    let len = s.len() as i32;

    let start = match i {
        Some(idx) if idx > 0 => cmp::min(idx - 1, len),
        Some(idx) if idx < 0 => cmp::max(len + idx, 0),
        _ => 0,
    };

    let substr = &s[start as usize..];

    let mut char_count = 0;
    for (byte_idx, _) in substr.char_indices() {
        if char_count == n {
            return Ok((start + byte_idx as i32) as i32);
        }
        char_count += 1;
    }

    Ok(-1)
}


// fn lua_utf8_codes(lua: &Lua, s: String) -> LuaResult<(mlua::Function, mlua::Table, mlua::Value)> {
//     let mut table = lua.create_table()?;

//     for c in s.chars() {
//         table.push_back(Value::Integer(c as u32 as i64));
//     }

//     let next = lua.globals().get("next")?;
//     oK((next, table, Value::Nil))
// }

// compact for luajit
pub fn register_lua_utf8(lua: &Lua) -> LuaResult<()> {
    let utf8 = lua.create_table()?;
    utf8.set("len", lua.create_function(lua_utf8_len)?)?;
    utf8.set("char", lua.create_function(lua_utf8_char)?)?;
    utf8.set("codepoint", lua.create_function(lua_utf8_codepoint)?)?;
    utf8.set("offset", lua.create_function(lua_utf8_offset)?)?;
    // utf8.set("codes", lua.create_function(lua_utf8_codes)?)?;
    lua.globals().set("utf8", utf8)?;
    Ok(())
}
