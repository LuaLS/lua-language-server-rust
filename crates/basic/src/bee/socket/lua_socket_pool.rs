use std::collections::HashMap;
use std::io;
use std::sync::Arc;
#[cfg(unix)]
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::Mutex;

use super::lua_socket::{LuaSocket, SocketStream, SocketStreamData, SocketType};

pub struct LuaSocketPool {
    socket_stream_pool: HashMap<i32, SocketStream>,
    socket_data_pool: HashMap<i32, Vec<SocketStreamData>>,
    id_counter: i32,
}

impl LuaSocketPool {
    pub fn new() -> LuaSocketPool {
        LuaSocketPool {
            socket_stream_pool: HashMap::new(),
            socket_data_pool: HashMap::new(),
            id_counter: 1,
        }
    }

    pub fn get_socket_stream(&mut self, fd: i32) -> Option<&mut SocketStream> {
        self.socket_stream_pool.get_mut(&fd)
    }

    pub fn insert_socket_stream(&mut self, fd: i32, socket_stream: SocketStream) {
        self.socket_stream_pool.insert(fd, socket_stream);
    }

    #[allow(unused)]
    pub fn remove_socket_stream(&mut self, fd: i32) {
        self.socket_stream_pool.remove(&fd);
    }

    pub fn get_socket_data(&mut self, fd: i32) -> Option<&mut Vec<SocketStreamData>> {
        self.socket_data_pool.get_mut(&fd)
    }

    pub fn insert_socket_data(&mut self, fd: i32, data: SocketStreamData) {
        let socket_data = self.socket_data_pool.entry(fd).or_insert(Vec::new());
        socket_data.push(data);
    }

    pub fn create_socket(&mut self, socket_type: SocketType) -> io::Result<LuaSocket> {
        let fd = self.id_counter;
        self.id_counter += 1;
        Ok(LuaSocket::new(socket_type, fd))
    }

    pub fn close_socket(&mut self, fd: i32) -> io::Result<()> {
        self.socket_stream_pool.remove(&fd);
        self.socket_data_pool.remove(&fd);
        Ok(())
    }

    pub async fn can_read(&mut self, fd: i32) -> io::Result<bool> {
        if !self.socket_stream_pool.contains_key(&fd) {
            return Ok(false);
        }
        match &self.socket_stream_pool.get_mut(&fd).unwrap() {
            SocketStream::Tcp(stream) => match stream.readable().await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            },
            SocketStream::TcpListener(tcp_listener) => match tcp_listener.accept().await {
                Ok(client) => {
                    let (stream, _) = client;
                    let socket = self.create_socket(SocketType::Tcp).unwrap();
                    self.insert_socket_stream(socket.fd, SocketStream::Tcp(stream));
                    let socket_data = SocketStreamData::Socket(socket);
                    self.insert_socket_data(fd, socket_data);
                    Ok(true)
                }
                Err(_) => Ok(false),
            },
            #[cfg(unix)]
            SocketStream::Unix(stream) => match stream.readable().await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            },
            #[cfg(unix)]
            SocketStream::UnixListener(unix_listener) => match unix_listener.accept().await {
                Ok(client) => {
                    let (stream, _) = client;
                    let socket = self.create_socket(SocketType::Unix).unwrap();
                    self.insert_socket_stream(fd, SocketStream::Unix(stream));
                    let socket_data = SocketStreamData::Socket(socket);
                    self.insert_socket_data(fd, socket_data);
                    Ok(true)
                }
                Err(_) => Ok(false),
            },
            _ => Ok(false),
        }
    }

    pub async fn can_write(&mut self, fd: i32) -> io::Result<bool> {
        if !self.socket_stream_pool.contains_key(&fd) {
            return Ok(false);
        }
        match &self.socket_stream_pool.get_mut(&fd).unwrap() {
            SocketStream::Tcp(stream) => match stream.writable().await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            },
            #[cfg(unix)]
            SocketStream::Unix(stream) => match stream.writable().await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            },
            _ => Ok(false),
        }
    }
}

lazy_static! {
    pub static ref SOCKET_POOL: Arc<Mutex<LuaSocketPool>> =
        Arc::new(Mutex::new(LuaSocketPool::new()));
}
