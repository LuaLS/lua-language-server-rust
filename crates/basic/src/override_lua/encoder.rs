use mlua::{ffi::lua, lua_State, prelude::*};    
use std::cmp;

enum EncoderEncoding {
    UTF8,
    UTF16,
    UTF16LE,
    UTF16BE,
}

enum EncoderBom {
    No,
    Yes,
    Auto,
}

fn get_encoder_encoding(encoding: &str) -> Option<EncoderEncoding> {
    match encoding {
        "utf8" => Some(EncoderEncoding::UTF8),
        "utf16" => Some(EncoderEncoding::UTF16),
        "utf16le" => Some(EncoderEncoding::UTF16LE),
        "utf16be" => Some(EncoderEncoding::UTF16BE),
        _ => None,
    }
}

fn get_encoder_bom(bom: &str) -> Option<EncoderBom> {
    match bom {
        "no" => Some(EncoderBom::No),
        "yes" => Some(EncoderBom::Yes),
        "auto" => Some(EncoderBom::Auto),
        _ => None,
    }
}

fn encoder_len(lua: &Lua, (encoding, s, i, j): (String, String, Option<i32>, Option<i32>)) -> LuaResult<i32> {
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

    match get_encoder_encoding(&encoding) {
        Some(EncoderEncoding::UTF8) => Ok(substr.len() as i32),
        Some(EncoderEncoding::UTF16) | Some(EncoderEncoding::UTF16LE) | Some(EncoderEncoding::UTF16BE) => {
            Ok(substr.encode_utf16().count() as i32)
        }
        None => Err(mlua::Error::RuntimeError("Unsupported encoding".to_string())),
    }
}

fn encoder_offset(lua: &Lua, (encoding, s, n, i): (String, String, i32, Option<i32>)) -> LuaResult<i32> {
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

fn encoder_encode(lua: &Lua, (encoding, s, bom): (String, String, String)) -> LuaResult<String> {
    let bom = get_encoder_bom(&bom).ok_or_else(|| mlua::Error::RuntimeError("Invalid BOM option".to_string()))?;
    let encoding = get_encoder_encoding(&encoding).ok_or_else(|| mlua::Error::RuntimeError("Unsupported encoding".to_string()))?;

    let mut encoded: Vec<u8> = match encoding {
        EncoderEncoding::UTF8 => s.into_bytes(),
        EncoderEncoding::UTF16 | EncoderEncoding::UTF16LE => s.encode_utf16().flat_map(|u| u.to_le_bytes()).collect(),
        EncoderEncoding::UTF16BE => s.encode_utf16().flat_map(|u| u.to_be_bytes()).collect(),
    };

    if let EncoderBom::Yes = bom {
        match encoding {
            EncoderEncoding::UTF8 => encoded.splice(0..0, [0xEF, 0xBB, 0xBF].iter().cloned()).for_each(drop),
            EncoderEncoding::UTF16 | EncoderEncoding::UTF16LE => encoded.splice(0..0, [0xFF, 0xFE].iter().cloned()).for_each(drop),
            EncoderEncoding::UTF16BE => encoded.splice(0..0, [0xFE, 0xFF].iter().cloned()).for_each(drop),
        }
    }

    Ok(encoded.iter().map(|b| *b as char).collect())
}

fn encoder_decode(lua: &Lua, (encoding, s): (String, String)) -> LuaResult<String> {
    let encoding = get_encoder_encoding(&encoding).ok_or_else(|| mlua::Error::RuntimeError("Unsupported encoding".to_string()))?;

    let decoded = match encoding {
        EncoderEncoding::UTF8 => s,
        EncoderEncoding::UTF16 | EncoderEncoding::UTF16LE => {
            let bytes: Vec<u16> = s.as_bytes().chunks(2).map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]])).collect();
            String::from_utf16(&bytes).map_err(|e| mlua::Error::RuntimeError(e.to_string()))?
        }
        EncoderEncoding::UTF16BE => {
            let bytes: Vec<u16> = s.as_bytes().chunks(2).map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]])).collect();
            String::from_utf16(&bytes).map_err(|e| mlua::Error::RuntimeError(e.to_string()))?
        }
    };

    Ok(decoded)
}

pub fn lua_encoder_loader(lua: &Lua) -> LuaResult<LuaTable> {
    let encoder = lua.create_table()?;
    encoder.set("len", lua.create_function(encoder_len)?);
    encoder.set("offset", lua.create_function(encoder_offset)?);
    encoder.set("encode", lua.create_function(encoder_encode)?);
    encoder.set("decode", lua.create_function(encoder_decode)?);
    Ok(encoder)
}