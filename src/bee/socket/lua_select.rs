use mlua::prelude::LuaResult;
use mlua::prelude::*;
use std::borrow::BorrowMut;
// use std::cell::RefCell;
use std::collections::HashMap;
// use std::rc::Rc;
// use std::result;
use tokio::select;

use super::lua_socket::LuaSocket;

pub struct LuaSelect {
    sockets: HashMap<i32, mlua::UserDataRefMut<LuaSocket>>,
    callbacks: HashMap<i32, (mlua::Function, i32)>,
    id_counter: i32,
}

impl LuaSelect {
    pub fn new() -> LuaSelect {
        LuaSelect {
            callbacks: HashMap::new(),
            sockets: HashMap::new(),
            id_counter: 1,
        }
    }

    async fn wait(&mut self, time: u64) -> LuaResult<Vec<(mlua::Function, i32)>> {
        let mut rest_time = time;
        let mut result = Vec::new();

        let callbacks: Vec<(i32, mlua::Function, i32)> = self
            .callbacks
            .iter()
            .map(|(socket_id, (callback, flag))| (*socket_id, callback.clone(), *flag))
            .collect();
        for (socket_id, callback, flag) in callbacks {
            if let Some(socket) = self.sockets.get_mut(&socket_id) {
                if flag & 0x01 != 0 {
                    select! {
                        _ = socket.can_read() => {
                            result.push((callback.clone(), 0x01));
                        }
                        _ = tokio::time::sleep(tokio::time::Duration::from_millis(1)) => {}
                    }
                    rest_time -= 1;
                }

                if flag & 0x02 != 0 {
                    select! {
                        _ = socket.can_write() => {
                            result.push((callback.clone(), 0x02));
                        }
                        _ = tokio::time::sleep(tokio::time::Duration::from_millis(1)) => {}
                    }
                    rest_time -= 1;
                }
            }
        }

        if (result.len() == 0) && (rest_time > 0) {
            tokio::time::sleep(tokio::time::Duration::from_millis(rest_time)).await;
        }

        Ok(result)
    }

    fn event_add(
        &mut self,
        mut socket: mlua::UserDataRefMut<LuaSocket>,
        flag: i32,
        callback: mlua::Function,
    ) -> LuaResult<()> {
        self.may_add_id(&mut socket);
        self.callbacks.insert(socket.fd, (callback, flag));
        self.sockets.insert(socket.fd, socket);
        Ok(())
    }

    fn event_mod(
        &mut self,
        mut socket: mlua::UserDataRefMut<LuaSocket>,
        flag: i32,
    ) -> LuaResult<()> {
        self.may_add_id(&mut socket);
        if let Some((_, old_flag)) = self.callbacks.get_mut(&socket.fd) {
            *old_flag = flag;
        }
        Ok(())
    }

    fn event_del(&mut self, mut socket: mlua::UserDataRefMut<LuaSocket>) -> LuaResult<()> {
        self.may_add_id(&mut socket);
        self.sockets.remove(&socket.fd);
        self.callbacks.remove(&socket.fd);
        Ok(())
    }

    fn may_add_id(&mut self, socket: &mut mlua::UserDataRefMut<LuaSocket>) {
        let id = socket.fd;
        if id == 0 {
            self.id_counter += 1;
            socket.borrow_mut().fd = self.id_counter;
        }
    }
}

impl LuaUserData for LuaSelect {
    fn add_methods<'a, M: LuaUserDataMethods<'a, Self>>(methods: &mut M) {
        methods.add_async_method_mut("wait", |lua, this, time: u64| async move {
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
            let next_func = lua.globals().get::<_, LuaFunction>("next").unwrap();
            let table = lua.create_table().unwrap();
            for (callback, flag) in results {
                table.set(callback, flag).unwrap();
            }

            Ok((next_func, table, mlua::Nil))
        });
        methods.add_method_mut(
            "event_add",
            |_, this, (socekt, flag, callback): (mlua::UserDataRefMut<LuaSocket>, i32, mlua::Function)| {
                this.event_add(socekt, flag, callback)
            },
        );
        methods.add_method_mut(
            "event_mod",
            |_, this, (socekt, flag): (mlua::UserDataRefMut<LuaSocket>, i32)| {
                this.event_mod(socekt, flag)
            },
        );

        methods.add_method_mut(
            "event_del",
            |_, this, socekt: mlua::UserDataRefMut<LuaSocket>| this.event_del(socekt),
        );
    }
}
