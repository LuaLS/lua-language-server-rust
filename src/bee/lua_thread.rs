use crate::lua_preload;
use lazy_static::lazy_static;
use mlua::prelude::LuaResult;
use mlua::{prelude::*, Lua, UserData};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tokio::runtime::Builder;
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
    receivers: HashMap<i64, Arc<Mutex<mpsc::Receiver<i64>>>>,
    senders: HashMap<i64, Arc<Mutex<mpsc::Sender<i64>>>>,
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
        self.receivers.insert(id, Arc::new(Mutex::new(receiver)));
        self.senders.insert(id, Arc::new(Mutex::new(sender)));
    }

    pub fn get_channel(&self, name: &str) -> Option<LuaChannel> {
        self.channels.get(name).cloned()
    }

    pub fn get_sender(&self, id: i64) -> Option<Arc<Mutex<mpsc::Sender<i64>>>> {
        self.senders.get(&id).cloned()
    }

    pub fn get_receiver(&self, id: i64) -> Option<Arc<Mutex<mpsc::Receiver<i64>>>> {
        self.receivers.get(&id).cloned()
    }
}

lazy_static! {
    static ref ChannelMgr: Arc<Mutex<LuaChannelMgr>> = Arc::new(Mutex::new(LuaChannelMgr::new()));
}

impl UserData for LuaChannel {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(this.mt_string())
        });

        methods.add_async_method("push", |lua, this, args: mlua::MultiValue| async move {
            let id = this.id;
            let lua_seri_pack = lua.globals().get::<LuaFunction>("lua_seri_pack")?;
            let ptr = lua_seri_pack.call::<i64>(args).unwrap();
            let opt_sender = { ChannelMgr.lock().unwrap().get_sender(id) };
            if let Some(sender) = opt_sender {
                let sender = sender.lock().unwrap();
                sender.send(ptr).await.unwrap();
            }
            Ok(())
        });

        methods.add_async_method("pop", |lua, this, ()| async move {
            let id = this.id;
            let opt_receiver = { ChannelMgr.lock().unwrap().get_receiver(id) };
            if let Some(receiver) = opt_receiver {
                let data = receiver.lock().unwrap().recv().await;
                if let Some(data) = data {
                    let lua_seri_unpack = lua.globals().get::<LuaFunction>("lua_seri_unpack")?;
                    let mut returns = lua_seri_unpack.call::<mlua::MultiValue>(data).unwrap();
                    returns.insert(0, mlua::Value::Boolean(true));
                    return Ok(returns);
                }
            }

            let mut returns = mlua::MultiValue::new();
            returns.insert(0, mlua::Value::Boolean(false));
            Ok(returns)
        });

        methods.add_async_method("bpop", |lua, this, ()| async move {
            let id = this.id;
            let opt_receiver = { ChannelMgr.lock().unwrap().get_receiver(id) };
            if let Some(receiver) = opt_receiver {
                let data = receiver.lock().unwrap().recv().await;
                if let Some(data) = data {
                    let lua_seri_unpack = lua.globals().get::<LuaFunction>("lua_seri_unpack")?;
                    let returns = lua_seri_unpack.call::<mlua::MultiValue>(data).unwrap();
                    return Ok(returns);
                }
            }

            let returns = mlua::MultiValue::new();
            Ok(returns)
        });
    }
}

async fn bee_thread_sleep(_: Lua, time: u64) -> LuaResult<()> {
    tokio::time::sleep(Duration::from_millis(time)).await;
    Ok(())
}

fn bee_thread_newchannel(_: &Lua, name: String) -> LuaResult<()> {
    ChannelMgr.lock().unwrap().new_channel(name);
    Ok(())
}

fn bee_thread_channel(_: &Lua, name: String) -> LuaResult<LuaChannel> {
    let mut mgr = ChannelMgr.lock().unwrap();
    if let Some(channel) = mgr.get_channel(&name) {
        Ok(channel)
    } else {
        mgr.new_channel(name.to_string());
        if let Some(channel) = mgr.get_channel(&name) {
            return Ok(channel);
        }
        Err(mlua::Error::RuntimeError("Channel not found".to_string()))
    }
}

fn bee_thread_thread(_: &Lua, script: String) -> LuaResult<()> {
    thread::spawn(move || {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(async move {
            let lua = unsafe { Lua::unsafe_new() };
            if let Err(e) = lua_preload::lua_preload(&lua) {
                eprintln!("Error during lua_preload: {:?}", e);
                return;
            }
            lua.load(script.as_bytes())
                .call_async::<()>(())
                .await
                .unwrap();
        });
    });
    Ok(())
}

pub fn bee_thread(lua: &Lua) -> LuaResult<LuaTable> {
    let thread = lua.create_table()?;
    thread.set("sleep", lua.create_async_function(bee_thread_sleep)?)?;
    thread.set("newchannel", lua.create_function(bee_thread_newchannel)?)?;
    thread.set("channel", lua.create_function(bee_thread_channel)?)?;
    thread.set("thread", lua.create_function(bee_thread_thread)?)?;
    Ok(thread)
}
