use std::{
    io::Read,
    net::{SocketAddr, TcpStream},
};

use interfaces::models::Settings;
use log::error;

pub struct Session {
    stream: TcpStream,
    addr: SocketAddr,
}

impl Session {
    pub(crate) fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        Self { stream, addr }
    }

    pub fn process(&mut self, settings: &Settings) {
        let _ = settings;
        let mut data = Vec::new();
        match &self.stream.read_to_end(&mut data) {
            Ok(_) => {}
            Err(err) => error!("Error reading msg from {}: {err}", self.addr),
        };
    }

    pub(crate) fn stop(&self) {
        let _ = self.stream.shutdown(std::net::Shutdown::Both);
    }
}
