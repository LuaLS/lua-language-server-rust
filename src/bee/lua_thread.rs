use lazy_static::lazy_static;
use mlua::ffi::lua_pushboolean;
use mlua::prelude::LuaResult;
use mlua::{ffi, lua_State, prelude::*, Lua, MetaMethod, Table, UserData, UserDataMethods};
use std::collections::{HashMap, VecDeque};
use std::ffi::{c_void, CStr};
use std::os::raw::c_int;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread, vec};

#[derive(Clone, Debug)]
pub struct LuaChannel {
    name: String,
}

impl LuaChannel {
    fn new(name: String) -> LuaChannel {
        LuaChannel { name }
    }
}

pub struct LuaChannelMgr {
    channels: HashMap<String, LuaChannel>,
    channel_data: HashMap<String, VecDeque<i64>>,
}

impl LuaChannelMgr {
    pub fn new() -> LuaChannelMgr {
        LuaChannelMgr {
            channels: HashMap::new(),
            channel_data: HashMap::new(),
        }
    }

    pub fn new_channel(&mut self, name: String) {
        let channel = LuaChannel::new(name.clone());
        self.channels.insert(name, channel);
    }

    pub fn get_channel(&self, name: &str) -> Option<LuaChannel> {
        self.channels.get(name).cloned()
    }

    pub fn push(&mut self, name: &str, data: i64) {
        if let Some(queue) = self.channel_data.get_mut(name) {
            queue.push_back(data);
        } else {
            let mut queue = VecDeque::new();
            queue.push_back(data);
            self.channel_data.insert(name.to_string(), queue);
        }
    }

    pub fn pop(&mut self, name: &str) -> Option<i64> {
        if let Some(queue) = self.channel_data.get_mut(name) {
            queue.pop_front()
        } else {
            None
        }
    }
}

lazy_static! {
    static ref luaChannelMgr: Mutex<LuaChannelMgr> = Mutex::new(LuaChannelMgr::new());
}

extern "C-unwind" {
    fn seri_unpackptr(lua: *mut lua_State, buffer: *mut c_void) -> i32;

    fn seri_pack(lua: *mut lua_State, from: i32, sz: *mut c_int) -> *mut c_void;
}

impl UserData for LuaChannel {}

pub fn register_lua_channel<'lua>(lua: &'lua Lua) {
    lua.register_userdata_type::<LuaChannel>(|methods: _| unsafe {
        unsafe extern "C-unwind" fn lua_channel_push(lua_State: *mut lua_State) -> i32 {
            let top = ffi::lua_gettop(lua_State);
            if top < 2 {
                return 0;
            }

            let channel_name = {
                let value_value = ffi::lua_type(lua_State, 1);
                if value_value == ffi::LUA_TSTRING {
                    let value = ffi::lua_tostring(lua_State, 1);
                    let c_str = CStr::from_ptr(value);
                    c_str.to_str().unwrap()
                } else {
                    ""
                }
            };

            let buffer_id = seri_pack(lua_State, 1, std::ptr::null_mut());
            luaChannelMgr
                .lock()
                .unwrap()
                .push(&channel_name, buffer_id as i64);
            0
        }
        let lua_c_channel_push = unsafe { lua.create_c_function(lua_channel_push).unwrap() };
        methods.add_method(
            "push",
            move |_, this, args: mlua::MultiValue| -> LuaResult<()> {
                let name = this.name.clone();
                lua_c_channel_push.call::<_, ()>((name, args))?;
                Ok(())
            },
        );

        unsafe extern "C-unwind" fn lua_channel_pop(lua_State: *mut lua_State) -> i32 {
            let top = ffi::lua_gettop(lua_State);
            if top < 1 {
                return 0;
            }

            let channel_name = {
                let value_value = ffi::lua_type(lua_State, 1);
                if value_value == ffi::LUA_TSTRING {
                    let value = ffi::lua_tostring(lua_State, 1);
                    let c_str = CStr::from_ptr(value);
                    c_str.to_str().unwrap()
                } else {
                    ""
                }
            };

            if let Some(channel_data) = luaChannelMgr.lock().unwrap().pop(&channel_name) {
                ffi::lua_pushboolean(lua_State, 1);
                let count = seri_unpackptr(lua_State, channel_data as *mut c_void);
                1 + count
            } else {
                ffi::lua_pushboolean(lua_State, 0);
                1
            }
        }

        let lua_c_channel_pop = unsafe { lua.create_c_function(lua_channel_pop).unwrap() };
        methods.add_method("pop", move |_, this, ()| -> LuaResult<mlua::MultiValue> {
            let name = this.name.clone();
            let returns = lua_c_channel_pop.call::<_, mlua::MultiValue>(name)?;
            Ok(returns)
        });

        unsafe extern "C-unwind" fn lua_channel_bpop(lua_State: *mut lua_State) -> i32 {
            let top = ffi::lua_gettop(lua_State);
            if top < 1 {
                return 0;
            }

            let channel_name = {
                let value_value = ffi::lua_type(lua_State, 1);
                if value_value == ffi::LUA_TSTRING {
                    let value = ffi::lua_tostring(lua_State, 1);
                    let c_str = CStr::from_ptr(value);
                    c_str.to_str().unwrap()
                } else {
                    ""
                }
            };

            if let Some(channel_data) = luaChannelMgr.lock().unwrap().pop(&channel_name) {
                let count = seri_unpackptr(lua_State, channel_data as *mut c_void);
                count
            } else {
                0
            }
        }

        let lua_c_channel_bpop = unsafe { lua.create_c_function(lua_channel_bpop).unwrap() };
        methods.add_method("bpop", move |_, this, ()| -> LuaResult<mlua::MultiValue> {
            let name = this.name.clone();
            let returns = lua_c_channel_bpop.call::<_, mlua::MultiValue>(name)?;
            Ok(returns)
        });
    });

    // methods.add_method("pop", |_, this, ()| Ok(this.blocked_pop()));
    // methods.add_method("bpop", |_, this, timeout: Duration| {
    //     Ok(this.timed_pop(timeout))
    // });
}

fn bee_thread_sleep(_: &Lua, time: u64) -> LuaResult<()> {
    thread::sleep(Duration::from_millis(time));
    Ok(())
}

fn bee_thread_newchannel(_: &Lua, name: String) -> LuaResult<()> {
    luaChannelMgr.lock().unwrap().new_channel(name);
    Ok(())
}

fn bee_thread_channel(_: &Lua, name: String) -> LuaResult<LuaChannel> {
    if let Some(channel) = luaChannelMgr.lock().unwrap().get_channel(&name) {
        Ok(channel)
    } else {
        Err(mlua::Error::RuntimeError(format!(
            "Channel {} not found",
            name
        )))
    }
}

pub fn bee_thread(lua: &Lua) -> LuaResult<LuaTable> {
    let thread = lua.create_table()?;
    thread.set(
        "sleep",
        lua.create_function(|lua, time: u64| Ok(bee_thread_sleep(lua, time)))?,
    )?;
    thread.set(
        "newchannel",
        lua.create_function(|lua, name: String| Ok(bee_thread_newchannel(lua, name)))?,
    )?;
    thread.set(
        "channel",
        lua.create_function(|lua, name: String| Ok(bee_thread_channel(lua, name)))?,
    )?;
    Ok(thread)
}
