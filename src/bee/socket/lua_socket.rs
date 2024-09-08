use mlua::prelude::LuaResult;
use mlua::prelude::*;
use std::borrow::BorrowMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
#[cfg(unix)]
use tokio::net::{UnixListener, UnixStream};

enum SocketStream {
    None,
    Tcp(TcpStream),
    TcpListener(TcpListener),
    #[cfg(unix)]
    Unix(UnixStream),
    #[cfg(unix)]
    UnixListener(UnixListener),
}

enum SocketStreamData {
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
    socket_stream: SocketStream,
    pub fd: i32,
    socket_data: Option<Vec<SocketStreamData>>,
}

impl LuaSocket {
    pub fn new(socket_type: SocketType) -> LuaSocket {
        LuaSocket {
            socket_type,
            socket_stream: SocketStream::None,
            fd: 0,
            socket_data: None,
        }
    }

    fn close(&mut self) -> LuaResult<()> {
        match self.socket_stream.borrow_mut() {
            SocketStream::Tcp(stream) => {
                drop(stream);
            }
            SocketStream::TcpListener(stream) => {
                drop(stream);
            }
            #[cfg(unix)]
            SocketStream::Unix(stream) => {
                drop(stream);
            }
            #[cfg(unix)]
            SocketStream::UnixListener(stream) => {
                drop(stream);
            }
            SocketStream::None => {}
        }

        Ok(())
    }

    async fn send(&mut self, data: String) -> LuaResult<()> {
        match self.socket_stream.borrow_mut() {
            SocketStream::Tcp(stream) => {
                stream.write_all(data.as_bytes()).await?;
            }
            #[cfg(unix)]
            SocketStream::Unix(stream) => {
                stream.write_all(data.as_bytes()).await?;
            }
            _ => return Err(mlua::Error::RuntimeError("Invalid fd".to_string())),
        }
        Ok(())
    }

    async fn recv(&mut self) -> LuaResult<String> {
        let mut buf = vec![0; 1024];
        match self.socket_stream.borrow_mut() {
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

    async fn bind(&mut self, addr: String, port: i32) -> LuaResult<()> {
        match self.socket_type {
            SocketType::Tcp => {
                let addr = format!("{}:{}", addr, port);
                let listener = TcpListener::bind(addr).await?;
                self.socket_stream = SocketStream::TcpListener(listener);
                Ok(())
            }
            #[cfg(unix)]
            SocketType::Unix => {
                let listener = UnixListener::bind(addr).await?;
                self.socket_stream = SocketStream::UnixListener(listener);
                Ok(())
            }
            _ => Err(mlua::Error::RuntimeError("Invalid fd".to_string())),
        }
    }

    fn listen(&mut self) -> LuaResult<()> {
        self.socket_data = Some(Vec::new());
        Ok(())
    }

    async fn connect(&mut self, addr: String, port: i32) -> LuaResult<()> {
        match self.socket_type {
            SocketType::Tcp => {
                let addr = format!("{}:{}", addr, port);
                let stream = TcpStream::connect(addr).await?;
                self.socket_stream = SocketStream::Tcp(stream);
                Ok(())
            }
            #[cfg(unix)]
            SocketType::Unix => {
                let stream = UnixStream::connect(addr).await?;
                self.socket_stream = SocketStream::Unix(stream);
                Ok(())
            }
            _ => Err(mlua::Error::RuntimeError("Invalid fd".to_string())),
        }
    }

    async fn accept(&mut self) -> LuaResult<LuaSocket> {
        match self.socket_stream.borrow_mut() {
            SocketStream::TcpListener(listener) => {
                let (stream, _) = listener.accept().await?;
                Ok(LuaSocket {
                    socket_type: SocketType::Tcp,
                    socket_stream: SocketStream::Tcp(stream),
                    fd: 0,
                    socket_data: None,
                })
            }
            #[cfg(unix)]
            SocketStream::UnixListener(listener) => {
                let (stream, _) = listener.accept().await?;
                Ok(LuaSocket {
                    socket_type: SocketType::Unix,
                    socket_stream: SocketStream::Unix(stream),
                    fd: 0,
                    socket_data: None,
                })
            }
            _ => Err(mlua::Error::RuntimeError("Invalid fd".to_string())),
        }
    }

    fn status(&self) -> LuaResult<bool> {
        match self.socket_stream {
            SocketStream::Tcp(_) => Ok(true),
            #[cfg(unix)]
            SocketStream::Unix(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub async fn can_read(&mut self) -> LuaResult<bool> {
        match &self.socket_stream {
            SocketStream::Tcp(stream) => match stream.readable().await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            },
            SocketStream::TcpListener(tcp_listener) => match tcp_listener.accept().await {
                Ok(client) => {
                    let (stream, _) = client;
                    if let Some(socket_data) = &mut self.socket_data {
                        socket_data.push(SocketStreamData::Socket(LuaSocket {
                            socket_type: SocketType::Tcp,
                            socket_stream: SocketStream::Tcp(stream),
                            fd: 0,
                            socket_data: None,
                        }));
                    }
                    Ok(true)
                }
                Err(_) => Ok(false),
            },
            #[cfg(unix)]
            SocketStream::Unix(stream) => {
                let mut buf = vec![0; 1];
                match stream.try_read(&mut buf) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
            #[cfg(unix)]
            SocketStream::UnixListener(unix_listener) => match unix_listener.accept().await {
                Ok(client) => {
                    let (stream, _) = client;
                    if let Some(socket_data) = &mut self.socket_data {
                        socket_data.push(SocketStreamData::Socket(LuaSocket {
                            socket_type: SocketType::Unix,
                            socket_stream: SocketStream::Unix(stream),
                            fd: 0,
                            socket_data: None,
                        }));
                    }
                    Ok(true)
                }
                Err(_) => Ok(false),
            },
            _ => Ok(false),
        }
    }

    pub async fn can_write(&mut self) -> LuaResult<bool> {
        match &self.socket_stream {
            SocketStream::Tcp(stream) => match stream.writable().await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            },
            #[cfg(unix)]
            SocketStream::Unix(stream) => match stream.writable().await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
            _ => Ok(false),
        }
    }
}

impl LuaUserData for LuaSocket {
    fn add_methods<'a, M: LuaUserDataMethods<'a, Self>>(methods: &mut M) {
        methods.add_method_mut("close", |_, this, ()| this.close());
        methods.add_async_method_mut("send", |_, this, data: String| async move {
            this.send(data).await
        });
        methods.add_async_method_mut("recv", |_, this, ()| async move { this.recv().await });
        methods.add_async_method_mut("bind", |_, this, args: mlua::MultiValue| async move {
            let addr = args.get(1).unwrap().to_string()?;
            let port = match args.get(2) {
                Some(mlua::Value::Integer(p)) => p.clone().try_into().unwrap_or(0),
                _ => 0,
            };

            this.bind(addr, port).await
        });
        methods.add_method_mut("listen", |_, this, ()| this.listen());
        methods.add_async_method_mut("connect", |_, this, args: mlua::MultiValue| async move {
            let addr = args.get(1).unwrap().to_string()?;
            let port = match args.get(2) {
                Some(mlua::Value::Integer(p)) => p.clone().try_into().unwrap_or(0),
                _ => 0,
            };
            this.connect(addr, port).await
        });
        methods.add_async_method_mut("accept", |_, this, ()| async move { this.accept().await });
        methods.add_method("status", |_, this, ()| this.status());
    }
}
