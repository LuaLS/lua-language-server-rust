use mlua::prelude::LuaResult;
use mlua::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
#[cfg(unix)]
use tokio::net::{UnixListener, UnixStream};

use super::lua_socket_pool::SOCKET_POOL;

pub enum SocketStream {
    None,
    Tcp(TcpStream),
    TcpListener(TcpListener),
    #[cfg(unix)]
    Unix(UnixStream),
    #[cfg(unix)]
    UnixListener(UnixListener),
}

pub enum SocketStreamData {
    Socket(LuaSocket),
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum SocketType {
    Tcp,
    Unix,
}

pub struct LuaSocket {
    socket_type: SocketType,
    pub fd: i32,
}

impl LuaSocket {
    pub fn new(socket_type: SocketType, fd: i32) -> LuaSocket {
        LuaSocket { socket_type, fd }
    }

    async fn close(&mut self) -> LuaResult<()> {
        SOCKET_POOL.lock().await.close_socket(self.fd)?;
        Ok(())
    }

    async fn send(&mut self, data: String) -> LuaResult<i32> {
        let len = data.len();
        match SOCKET_POOL.lock().await.get_socket_stream(self.fd).unwrap() {
            SocketStream::Tcp(stream) => {
                stream.write_all(data.as_bytes()).await?;
            }
            #[cfg(unix)]
            SocketStream::Unix(stream) => {
                stream.write_all(data.as_bytes()).await?;
            }
            _ => return Err(mlua::Error::RuntimeError("Invalid fd".to_string())),
        }
        Ok(len as i32)
    }

    async fn recv(&mut self) -> LuaResult<String> {
        let mut buf = vec![0; 1024];
        match SOCKET_POOL.lock().await.get_socket_stream(self.fd).unwrap() {
            SocketStream::Tcp(stream) => {
                let n = stream.read(&mut buf).await?;
                Ok(String::from_utf8_lossy(&buf[..n]).to_string())
            }
            #[cfg(unix)]
            SocketStream::Unix(stream) => {
                let n = stream.read(&mut buf).await?;
                Ok(String::from_utf8_lossy(&buf[..n]).to_string())
            }
            _ => Err(mlua::Error::RuntimeError("Invalid fd".to_string())),
        }
    }

    async fn bind(&mut self, addr: String, port: i32) -> LuaResult<bool> {
        match self.socket_type {
            SocketType::Tcp => {
                let addr = format!("{}:{}", addr, port);
                let listener = TcpListener::bind(addr).await?;
                let stream = SocketStream::TcpListener(listener);
                SOCKET_POOL
                    .lock()
                    .await
                    .insert_socket_stream(self.fd, stream);
                Ok(true)
            }
            #[cfg(unix)]
            SocketType::Unix => {
                let listener = UnixListener::bind(addr).await?;
                let stream = SocketStream::UnixListener(listener);
                SOCKET_POOL
                    .lock()
                    .await
                    .insert_socket_stream(self.fd, stream);
                Ok(true)
            }
            _ => Err(mlua::Error::RuntimeError("Invalid fd".to_string())),
        }
    }

    fn listen(&self) -> LuaResult<bool> {
        Ok(true)
    }

    async fn connect(&mut self, addr: String, port: i32) -> LuaResult<()> {
        match self.socket_type {
            SocketType::Tcp => {
                let addr = format!("{}:{}", addr, port);
                let stream = TcpStream::connect(addr).await?;
                let stream = SocketStream::Tcp(stream);
                SOCKET_POOL
                    .lock()
                    .await
                    .insert_socket_stream(self.fd, stream);
                Ok(())
            }
            #[cfg(unix)]
            SocketType::Unix => {
                let stream = UnixStream::connect(addr).await?;
                let stream = SocketStream::Unix(stream);
                SOCKET_POOL
                    .lock()
                    .await
                    .insert_socket_stream(self.fd, stream);
                Ok(())
            }
            _ => Err(mlua::Error::RuntimeError("Invalid fd".to_string())),
        }
    }

    async fn accept(&mut self) -> LuaResult<LuaSocket> {
        let mut socket_pool = SOCKET_POOL.lock().await;
        if let Some(socket_data) = socket_pool.get_socket_data(self.fd) {
            if socket_data.len() > 0 {
                if let SocketStreamData::Socket(socket) = socket_data.remove(0) {
                    return Ok(socket);
                }
            }
        }

        match socket_pool.get_socket_stream(self.fd).unwrap() {
            SocketStream::TcpListener(listener) => {
                let (stream, _) = listener.accept().await?;
                let socket = socket_pool.create_socket(SocketType::Tcp).unwrap();
                let stream = SocketStream::Tcp(stream);
                socket_pool.insert_socket_stream(socket.fd, stream);
                Ok(socket)
            }
            #[cfg(unix)]
            SocketStream::UnixListener(listener) => {
                let (stream, _) = listener.accept().await?;
                let socket = socket_pool.create_socket(SocketType::Unix).unwrap();
                let stream = SocketStream::Unix(stream);
                socket_pool.insert_socket_stream(socket.fd, stream);
                Ok(socket)
            }
            _ => Err(mlua::Error::RuntimeError("Invalid fd".to_string())),
        }
    }

    async fn status(&self) -> LuaResult<bool> {
        match SOCKET_POOL.lock().await.get_socket_stream(self.fd) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

impl LuaUserData for LuaSocket {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method_mut("close", |_, mut this, ()| async move { this.close().await });
        methods.add_async_method_mut("send", |_, mut this, data: String| async move {
            this.send(data).await
        });
        methods.add_async_method_mut("recv", |_, mut this, ()| async move { this.recv().await });
        methods.add_async_method_mut(
            "bind",
            |_, mut this, (addr, port): (String, Option<i32>)| async move {
                let port = port.unwrap_or(0);
                this.bind(addr, port).await
            },
        );
        methods.add_method_mut("listen", |_, this, ()| this.listen());
        methods.add_async_method_mut(
            "connect",
            |_, mut this, args: mlua::MultiValue| async move {
                let addr = args.get(1).unwrap().to_string()?;
                let port = match args.get(2) {
                    Some(mlua::Value::Integer(p)) => p.clone().try_into().unwrap_or(0),
                    _ => 0,
                };
                this.connect(addr, port).await
            },
        );
        methods.add_async_method_mut(
            "accept",
            |_, mut this, ()| async move { this.accept().await },
        );
        methods.add_async_method("status", |_, this, ()| async move { this.status().await });
    }
}
