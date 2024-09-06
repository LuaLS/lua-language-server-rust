use mlua::prelude::LuaResult;
use mlua::{FromLua, Lua, MetaMethod, Table, UserData, UserDataMethods};
#[cfg(unix)]
use std::os::unix::fs::symlink;

#[cfg(windows)]
use std::os::windows::fs::symlink_file;

#[derive(Clone, Debug)]
pub struct LuaFilePath {
    path: String,
}

impl LuaFilePath {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn filename(&self) -> Option<LuaFilePath> {
        std::path::Path::new(&self.path)
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| LuaFilePath::new(s.to_string()))
    }

    pub fn parent_path(&self) -> Option<LuaFilePath> {
        std::path::Path::new(&self.path)
            .parent()
            .and_then(|s| s.to_str())
            .map(|s| LuaFilePath::new(s.to_string()))
    }

    pub fn stem(&self) -> Option<LuaFilePath> {
        std::path::Path::new(&self.path)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| LuaFilePath::new(s.to_string()))
    }

    pub fn extension(&self) -> Option<String> {
        std::path::Path::new(&self.path)
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
    }

    pub fn is_absolute(&self) -> bool {
        std::path::Path::new(&self.path).is_absolute()
    }

    pub fn is_relative(&self) -> bool {
        std::path::Path::new(&self.path).is_relative()
    }

    pub fn remove_filename(&mut self) {
        if let Some(parent) = std::path::Path::new(&self.path).parent() {
            self.path = parent.to_str().unwrap_or("").to_string();
        }
    }

    pub fn replace_filename(&mut self, new_filename: &str) {
        let mut path = std::path::PathBuf::from(&self.path);
        path.set_file_name(new_filename);
        self.path = path.to_str().unwrap_or("").to_string();
    }

    pub fn replace_extension(&mut self, new_extension: &str) {
        let mut path = std::path::PathBuf::from(&self.path);
        path.set_extension(new_extension);
        self.path = path.to_str().unwrap_or("").to_string();
    }

    pub fn lexically_normal(&self) -> LuaFilePath {
        LuaFilePath::new(
            std::path::Path::new(&self.path)
                .components()
                .as_path()
                .to_str()
                .unwrap_or("")
                .to_string(),
        )
    }

    pub fn mt_tostring(&self) -> String {
        self.path.clone()
    }

    pub fn mi_div(&self, rhs: &str) -> LuaFilePath {
        LuaFilePath::new(
            std::path::Path::new(&self.path)
                .join(rhs)
                .to_str()
                .unwrap_or("")
                .to_string(),
        )
    }

    pub fn mt_concat(&self, rhs: &str) -> LuaFilePath {
        LuaFilePath::new(format!("{}{}", self.path, rhs))
    }

    pub fn mt_eq(&self, rhs: &LuaFilePath) -> bool {
        self.path == rhs.path
    }
}

impl UserData for LuaFilePath {
    fn add_methods<'a, M: UserDataMethods<'a, Self>>(methods: &mut M) {
        methods.add_method("string", |_, this, ()| Ok(this.path.clone()));
        methods.add_method("filename", |_, this, ()| Ok(this.filename()));
        methods.add_method("parent_path", |_, this, ()| Ok(this.parent_path()));
        methods.add_method("stem", |_, this, ()| Ok(this.stem()));
        methods.add_method("extension", |_, this, ()| Ok(this.extension()));
        methods.add_method("is_absolute", |_, this, ()| Ok(this.is_absolute()));
        methods.add_method("is_relative", |_, this, ()| Ok(this.is_relative()));
        methods.add_method_mut("remove_filename", |_, this, ()| {
            this.remove_filename();
            Ok(())
        });
        methods.add_method_mut("replace_filename", |_, this, new_filename: String| {
            this.replace_filename(&new_filename);
            Ok(())
        });
        methods.add_method_mut("replace_extension", |_, this, new_extension: String| {
            this.replace_extension(&new_extension);
            Ok(())
        });
        methods.add_method(
            "lexically_normal",
            |_, this, ()| Ok(this.lexically_normal()),
        );

        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.mt_tostring()));
        methods.add_meta_method(
            MetaMethod::Div,
            |_, this, rhs: String| Ok(this.mi_div(&rhs)),
        );
        methods.add_meta_method(MetaMethod::Concat, |_, this, rhs: String| {
            Ok(this.mt_concat(&rhs))
        });
        methods.add_meta_method(MetaMethod::Eq, |_, this, rhs: LuaFilePath| {
            Ok(this.mt_eq(&rhs))
        });
    }
}

impl FromLua for LuaFilePath {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => Ok(ud.borrow::<LuaFilePath>()?.clone()),
            mlua::Value::String(s) => Ok(LuaFilePath::new(s.to_str()?.to_string())),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "LuaFilePath",
                message: Some("value is not a LuaFilePath".to_string()),
            }),
        }
    }
}

fn path_constructor(_: &Lua, path: String) -> LuaResult<LuaFilePath> {
    Ok(LuaFilePath::new(path))
}

fn status(_: &Lua, path: String) -> LuaResult<()> {
    // Implementation for status function
    Ok(())
}

fn exists(_: &Lua, path: String) -> LuaResult<bool> {
    Ok(std::path::Path::new(&path).exists())
}

fn is_directory(_: &Lua, path: String) -> LuaResult<bool> {
    Ok(std::path::Path::new(&path).is_dir())
}

fn is_regular_file(_: &Lua, path: String) -> LuaResult<bool> {
    Ok(std::path::Path::new(&path).is_file())
}

fn file_size(_: &Lua, path: String) -> LuaResult<u64> {
    Ok(std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0))
}

fn create_directory(_: &Lua, path: String) -> LuaResult<()> {
    std::fs::create_dir(&path)?;
    Ok(())
}

fn create_directories(_: &Lua, path: String) -> LuaResult<()> {
    std::fs::create_dir_all(&path)?;
    Ok(())
}

fn rename(_: &Lua, (old_path, new_path): (String, String)) -> LuaResult<()> {
    std::fs::rename(&old_path, &new_path)?;
    Ok(())
}

fn remove(_: &Lua, path: String) -> LuaResult<()> {
    std::fs::remove_file(&path)?;
    Ok(())
}

fn remove_all(_: &Lua, path: String) -> LuaResult<()> {
    std::fs::remove_dir_all(&path)?;
    Ok(())
}

fn current_path(_: &Lua, (): ()) -> LuaResult<LuaFilePath> {
    Ok(LuaFilePath::new(
        std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_string(),
    ))
}

fn copy(_: &Lua, (source, destination): (String, String)) -> LuaResult<()> {
    std::fs::copy(&source, &destination)?;
    Ok(())
}

fn copy_file(_: &Lua, (source, destination): (String, String)) -> LuaResult<()> {
    std::fs::copy(&source, &destination)?;
    Ok(())
}

fn absolute(_: &Lua, path: String) -> LuaResult<LuaFilePath> {
    Ok(LuaFilePath::new(
        std::fs::canonicalize(&path)
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_string(),
    ))
}

fn canonical(_: &Lua, path: String) -> LuaResult<LuaFilePath> {
    Ok(LuaFilePath::new(
        std::fs::canonicalize(&path)
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_string(),
    ))
}

fn relative(_: &Lua, (from, to): (String, String)) -> LuaResult<LuaFilePath> {
    Ok(LuaFilePath::new(
        std::path::Path::new(&from)
            .join(&to)
            .to_str()
            .unwrap_or("")
            .to_string(),
    ))
}

fn last_write_time(_: &Lua, path: String) -> LuaResult<u64> {
    Ok(std::fs::metadata(&path)
        .map(|m| {
            m.modified()
                .unwrap()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        })
        .unwrap_or(0u64))
}

fn permissions(_: &Lua, path: String) -> LuaResult<u32> {
    Ok(0u32)
}

fn create_symlink(lua: &Lua, (source, destination): (String, String)) -> LuaResult<()> {
    #[cfg(unix)]
    {
        symlink(&source, &destination)?;
    }
    #[cfg(windows)]
    {
        symlink_file(&source, &destination)?;
    }
    Ok(())
}

fn create_directory_symlink(_: &Lua, (source, destination): (String, String)) -> LuaResult<()> {
    // Ok(std::os::unix::fs::symlink(&source, &destination)
    //     .map_err(|e| mlua::Error::ExternalError(Box::new(e)))?)
    Ok(())
}

fn create_hard_link(_: &Lua, (source, destination): (String, String)) -> LuaResult<()> {
    std::fs::hard_link(&source, &destination)?;
    Ok(())
}

fn temp_directory_path(_: &Lua, (): ()) -> LuaResult<LuaFilePath> {
    Ok(LuaFilePath::new(
        std::env::temp_dir().to_str().unwrap_or("").to_string(),
    ))
}

// fn pairs_ctor(_: &Lua, table: Table) -> LuaResult<mlua::Function> {
//     Ok(table.pairs())
// }

// fn pairs_r_ctor(_: &Lua, table: Table) -> LuaResult<mlua::Function> {
//     Ok(table.pairs())
// }

pub fn bee_filesystem(lua: &Lua) -> LuaResult<Table> {
    let exports = lua.create_table()?;

    exports.set("path", lua.create_function(path_constructor)?)?;
    exports.set("status", lua.create_function(status)?)?;
    exports.set("exists", lua.create_function(exists)?)?;
    exports.set("is_directory", lua.create_function(is_directory)?)?;
    exports.set("is_regular_file", lua.create_function(is_regular_file)?)?;
    exports.set("file_size", lua.create_function(file_size)?)?;
    exports.set("create_directory", lua.create_function(create_directory)?)?;
    exports.set(
        "create_directories",
        lua.create_function(create_directories)?,
    )?;
    exports.set("rename", lua.create_function(rename)?)?;
    exports.set("remove", lua.create_function(remove)?)?;
    exports.set("remove_all", lua.create_function(remove_all)?)?;
    exports.set("current_path", lua.create_function(current_path)?)?;
    exports.set("copy", lua.create_function(copy)?)?;
    exports.set("copy_file", lua.create_function(copy_file)?)?;
    exports.set("absolute", lua.create_function(absolute)?)?;
    exports.set("canonical", lua.create_function(canonical)?)?;
    exports.set("relative", lua.create_function(relative)?)?;
    exports.set("last_write_time", lua.create_function(last_write_time)?)?;
    exports.set("permissions", lua.create_function(permissions)?)?;
    exports.set("create_symlink", lua.create_function(create_symlink)?)?;
    exports.set(
        "create_directory_symlink",
        lua.create_function(create_directory_symlink)?,
    )?;
    exports.set("create_hard_link", lua.create_function(create_hard_link)?)?;
    exports.set(
        "temp_directory_path",
        lua.create_function(temp_directory_path)?,
    )?;
    // exports.set("pairs", lua.create_function(pairs_ctor)?)?;
    // exports.set("pairs_r", lua.create_function(pairs_r_ctor)?)?;

    Ok(exports)
}
