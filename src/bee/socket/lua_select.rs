use mlua::prelude::LuaResult;
use mlua::prelude::*;
use std::collections::HashMap;
use tokio::select;
use super::lua_socket::LuaSocket;
use super::lua_socket_pool::SOCKET_POOL;

pub struct LuaSelect {
    events: HashMap<i32, (mlua::Function, i32)>,
}

impl LuaSelect {
    pub fn new() -> LuaSelect {
        LuaSelect {
            events: HashMap::new(),
        }
    }

    async fn wait(&mut self, time: i32) -> LuaResult<Vec<(mlua::Function, i32)>> {
        let mut rest_time = time;
        let mut result = Vec::new();

        let callbacks: Vec<(i32, mlua::Function, i32)> = self
            .events
            .iter()
            .map(|(socket_id, (callback, flag))| (*socket_id, callback.clone(), *flag))
            .collect();
        {
            let mut socket_pool = SOCKET_POOL.lock().await;
            for (socket_id, callback, flag) in callbacks {
                if flag & 0x01 != 0 {
                    select! {
                        _ = socket_pool.can_read(socket_id) => {
                            result.push((callback.clone(), 0x01));
                        }
                        _ = tokio::time::sleep(tokio::time::Duration::from_millis(1)) => {}
                    }
                    rest_time -= 1;
                }

                if flag & 0x02 != 0 {
                    select! {
                        _ = socket_pool.can_write(socket_id) => {
                            result.push((callback.clone(), 0x02));
                        }
                        _ = tokio::time::sleep(tokio::time::Duration::from_millis(1)) => {}
                    }
                    rest_time -= 1;
                }
            }
        }

        if (result.len() == 0) && (rest_time > 0) {
            tokio::time::sleep(tokio::time::Duration::from_millis(rest_time as u64)).await;
        }

        Ok(result)
    }

    fn event_add(&mut self, fd: i32, flag: i32, callback: mlua::Function) -> LuaResult<()> {
        self.events.insert(fd, (callback, flag));
        Ok(())
    }

    fn event_mod(&mut self, fd: i32, flag: i32) -> LuaResult<()> {
        if let Some((_, old_flag)) = self.events.get_mut(&fd) {
            *old_flag = flag;
        }
        Ok(())
    }

    fn event_del(&mut self, fd: i32) -> LuaResult<()> {
        self.events.remove(&fd);
        Ok(())
    }
}

impl LuaUserData for LuaSelect {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method_mut("wait", |lua, mut this, time: i32| async move {
            let results: Vec<(LuaFunction, i32)> = this.wait(time).await.unwrap();
            // why this code is not working?
            // let iter = Rc::new(RefCell::new(results.into_iter()));
            // let iter_func = lua.create_function(move |_, ()| -> LuaResult<(mlua::Value, mlua::Value)> {
            //     let mut iter = *(iter.borrow_mut());

            //     if let Some((callback, flag)) = iter.next() {
            //         Ok((mlua::Value::Function(callback), mlua::Value::Integer(flag.into())))
            //     } else {
            //         Ok((mlua::Nil, mlua::Nil))
            //     }
            // })?;
            // Ok(iter_func)
            let next_func = lua.globals().get::<LuaFunction>("next").unwrap();
            let table = lua.create_table().unwrap();
            for (callback, flag) in results {
                table.set(callback, flag).unwrap();
            }

            Ok((next_func, table, mlua::Nil))
        });
        methods.add_method_mut(
            "event_add",
            |_, this, (socket, flag, callback): (mlua::UserDataRef<LuaSocket>, i32, mlua::Function)| {
                this.event_add(socket.fd, flag, callback)
            },
        );
        methods.add_method_mut(
            "event_mod",
            |_, this, (socket, flag): (mlua::UserDataRef<LuaSocket>, i32)| {
                this.event_mod(socket.fd, flag)
            },
        );

        methods.add_method_mut(
            "event_del",
            |_, this, socket: mlua::UserDataRef<LuaSocket>| this.event_del(socket.fd),
        );
    }
}
