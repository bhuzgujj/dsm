mod session;
mod events;

use std::{
	io::stdin,
	net::{IpAddr, TcpListener},
	str::FromStr,
	thread::{self},
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread::ScopedJoinHandle;
use clap::Args;
use interfaces::models::Settings;
use log::{debug, error};
use crate::serve::events::{ClientEvent, ServerEvent};
use crate::serve::session::Session;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialOrd, PartialEq)]
#[derive(Eq, Hash)]
enum HandleType {
	Event,
	SessionManager,
	Session(SocketAddr)
}

/// Host a local server to store datasets
#[derive(Args, Debug)]
pub struct Serve {
	/// Port to connect to
	#[clap(long, default_value = "43594")]
	port: u16,

	/// Host ip
	#[clap(long, default_value = "127.0.0.1")]
	host: String,
}

impl Serve {
	pub fn start(&self, settings: &Settings) -> anyhow::Result<()> {
		let addr = IpAddr::from_str(self.host.as_str())?;
		let socket = TcpListener::bind((addr, self.port))?;
		if let Err(err) = socket.set_nonblocking(true) {
			panic!("Cannot set nonblocking: {}", err);
		}
		println!("Starting server on {}:{}...", self.host, self.port);
		thread::scope(|scope| {
			let running = Arc::new(Mutex::new(true));
			let (sender, receiver) = channel::<ServerEvent>();
			let all_session = Arc::new(Mutex::new(HashMap::<SocketAddr, Sender<ClientEvent>>::new()));
			let scopes = Arc::new(Mutex::new(HashMap::<HandleType, ScopedJoinHandle<()>>::new()));
			let scopes_clone = scopes.clone();
			let server_sessions = all_session.clone();
			debug!("Starting event loop...");
			if let Ok(mut lock) = scopes.lock() {
				lock.insert(HandleType::Event, scope.spawn(move || {
					while let Ok(event) = receiver.recv() {
						match event {
							ServerEvent::Close => {
								match server_sessions.lock() {
									Ok(lock) => {
										for (_, session) in lock.iter() {
											if let Err(err) = session.send(ClientEvent::Close) {
												error!("{err}")
											}
										}
										break;
									}
									Err(err) => error!("{err}"),
								}
							},
							ServerEvent::CloseClient { addr } => {
								match server_sessions.lock() {
									Ok(mut sessions_lock) => {
										sessions_lock.remove(&addr);
										match scopes_clone.lock() {
											Ok(mut scopes_lock) => {
												scopes_lock.remove(&HandleType::Session(addr));
											}
											Err(err) => error!("{err}"),
										}
									}
									Err(err) => error!("{err}"),
								}
							}
						}
					}
					debug!("Event loop closed");
				}));
			}
			debug!("Started event loop");
			let sub_sessions = all_session.clone();
			let master_sender = sender.clone();
			let scopes_copy = scopes.clone();
			debug!("Starting listener...");
			if let Ok(mut lock) = scopes.lock() {
				let run = running.clone();
				lock.insert(HandleType::SessionManager, scope.spawn(move || {
					while *run.lock().unwrap() {
						match socket.accept() {
							Ok((stream, addr)) => {
								match sub_sessions.lock() {
									Ok(mut lock) => {
										let (session_sender, session_receiver) = channel::<ClientEvent>();
										lock.insert(addr.clone(), session_sender);
										let session = Session::new(stream, addr, session_receiver, master_sender.clone());
										if let Ok(mut lock) = scopes_copy.lock() {
											lock.insert(HandleType::Session(addr), scope.spawn(move || session.process(settings)));
										}
									}
									Err(err) => error!("{err}"),
								}
							},
							Err(err) => error!("{err}"),
						}
					}
				}));
			}
			debug!("Started listener");
			println!("Server is running... (exit command: 'exit')");
			let cli = stdin();
			while *running.lock().unwrap() {
				let mut cmd = String::new();
				let res = cli.read_line(&mut cmd);
				if res.is_err() || cmd.trim().to_lowercase() == "exit" {
					if let Ok(mut lock) = running.lock() {
						*lock = false;
					} else {
						panic!("Could not lock on the running");
					}
					if let Err(err) = sender.send(ServerEvent::Close) {
						panic!("{err}")
					}
					match scopes.lock() {
						Ok(mut lock) => {
							let keys = lock.keys().cloned().collect::<Vec<_>>();
							for key in keys {
								let scope = lock.remove(&key).unwrap();
								if let Err(err) = scope.join() {
									panic!("Could not join scope {err:?}");
								}
							}
						}
						Err(err) => panic!("Could not lock on scopes: {:?}", err),
					}
				}
			}
		});
		Ok(())
	}
}
