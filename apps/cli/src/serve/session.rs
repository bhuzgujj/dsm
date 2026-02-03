use std::{
	io::Read,
	net::{SocketAddr, TcpStream},
};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use interfaces::models::Settings;
use log::error;
use crate::serve::events::{ClientEvent, ServerEvent};

pub struct Session {
	stream: TcpStream,
	addr: SocketAddr,
	/// External events
	receiver: Receiver<ClientEvent>,
	/// Event to send back to the server
	notifier: Sender<ServerEvent>,
}

impl Session {
	pub(crate) fn new(stream: TcpStream, addr: SocketAddr, receiver: Receiver<ClientEvent>, notifier: Sender<ServerEvent>) -> Self {
		Self { stream, addr, receiver, notifier }
	}

	pub fn process(mut self, settings: &Settings) {
		println!("Connect {:?}", self.addr);
		let running = Arc::new(Mutex::new(true));
		let run = running.clone();
		let handle = std::thread::spawn(move || {
			self.stream.set_nonblocking(false).unwrap();
			self.stream.set_nodelay(true).unwrap();
			while *run.lock().unwrap() {
				let mut buffer = [0u8; 4];
				match &self.stream.read_exact(&mut buffer) {
					Ok(_) => {},
					Err(err) => error!("Error reading msg from {}: {err}", self.addr),
				};
				let len = u32::from_be_bytes(buffer) as usize;
				if len < 1 {
					self.notifier.send(ServerEvent::CloseClient { addr: self.addr }).unwrap();
					*run.lock().unwrap() = false;
					return;
				}
				let mut data = vec![0u8; len];
				match &self.stream.read_exact(&mut data) {
					Ok(_) => {
						println!("> {:?} > {}", self.addr, String::from_utf8_lossy(&data));
					},
					Err(err) => error!("Error reading msg from {}: {err}", self.addr),
				};
			}
			let _ = self.stream.shutdown(std::net::Shutdown::Both);
		});
		let _ = settings;
		while *running.lock().unwrap() {
			while let Ok(event) = self.receiver.try_recv() {
				match event {
					ClientEvent::Close => {
						*running.lock().unwrap() = false;
					}
				}
			}
		}
		if let Err(_) = handle.join() {
			error!("Error joining session thread");
		}
		println!("Disconnect {:?}", self.addr);
	}
}
