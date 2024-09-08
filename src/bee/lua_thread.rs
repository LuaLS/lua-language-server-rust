use lazy_static::lazy_static;
use mlua::prelude::LuaResult;
use mlua::{prelude::*, Lua, UserData};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub struct LuaChannel {
    name: String,
    id: i64,
}

impl LuaChannel {
    fn new(name: String, id: i64) -> LuaChannel {
        LuaChannel { name, id }
    }

    pub fn mt_string(&self) -> String {
        format!("Channel: {}", self.name.clone())
    }
}

pub struct LuaChannelMgr {
    channels: HashMap<String, LuaChannel>,
    receivers: HashMap<i64, mpsc::Receiver<i64>>,
    senders: HashMap<i64, mpsc::Sender<i64>>,
    id_counter: i64,
}

impl LuaChannelMgr {
    pub fn new() -> LuaChannelMgr {
        LuaChannelMgr {
            channels: HashMap::new(),
            receivers: HashMap::new(),
            senders: HashMap::new(),
            id_counter: 0,
        }
    }

    pub fn new_channel(&mut self, name: String) {
        let (sender, receiver) = mpsc::channel(100);
        let id = self.id_counter;
        self.id_counter += 1;
        let channel = LuaChannel::new(name.clone(), id);
        self.channels.insert(name.clone(), channel);
        self.receivers.insert(id, receiver);
        self.senders.insert(id, sender);
    }

    pub fn get_channel(&self, name: &str) -> Option<LuaChannel> {
        self.channels.get(name).cloned()
    }

    pub async fn push(&self, id: i64, data: i64) -> Result<(), mpsc::error::SendError<i64>> {
        if let Some(sender) = self.senders.get(&id) {
            sender.send(data).await
        } else {
            Err(mpsc::error::SendError(data))
        }
    }

    pub async fn pop(&mut self, id: i64) -> Option<i64> {
        if let Some(receiver) = self.receivers.get_mut(&id) {
            receiver.recv().await
        } else {
            None
        }
    }
}

lazy_static! {
    static ref luaChannelMgr: Mutex<LuaChannelMgr> = Mutex::new(LuaChannelMgr::new());
}

impl UserData for LuaChannel {
    fn add_methods<'a, M: LuaUserDataMethods<'a, Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(this.mt_string())
        });

        methods.add_async_method("push", |lua, this, args: mlua::MultiValue| async move {
            let id = this.id;
            let lua_seri_pack = lua.globals().get::<_, LuaFunction>("lua_seri_pack")?;
            let ptr = lua_seri_pack.call::<_, i64>(args).unwrap();
            luaChannelMgr.lock().unwrap().push(id, ptr).await.unwrap();
            Ok(())
        });

        methods.add_async_method("pop", |lua, this, ()| async move {
            let id = this.id;
            let data = luaChannelMgr.lock().unwrap().pop(id).await;
            if let Some(data) = data {
                let lua_seri_unpack = lua.globals().get::<_, LuaFunction>("lua_seri_unpack")?;
                let mut returns = lua_seri_unpack.call::<_, mlua::MultiValue>(data).unwrap();
                returns.insert(0, mlua::Value::Boolean(true));
                Ok(returns)
            } else {
                let mut returns = mlua::MultiValue::new();
                returns.insert(0, mlua::Value::Boolean(false));
                Ok(returns)
            }
        });

        methods.add_async_method("bpop", |lua, this, ()| async move {
            let id = this.id;
            let data = luaChannelMgr.lock().unwrap().pop(id).await;
            if let Some(data) = data {
                let lua_seri_unpack = lua.globals().get::<_, LuaFunction>("lua_seri_unpack")?;
                let returns = lua_seri_unpack.call::<_, mlua::MultiValue>(data).unwrap();
                Ok(returns)
            } else {
                Err(mlua::Error::RuntimeError("Channel is closed".to_string()))
            }
        });
    }
}

async fn bee_thread_sleep(_: &Lua, time: u64) -> LuaResult<()> {
    tokio::time::sleep(Duration::from_millis(time)).await;
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
    thread.set("sleep", lua.create_async_function(bee_thread_sleep)?)?;
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
