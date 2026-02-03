use std::io::{stdin, stdout, Read, Write};
use std::net::{IpAddr, Shutdown, TcpStream};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	/// Port to connect to
	#[clap(long, default_value = "43594")]
	port: u16,

	/// Host ip
	#[clap(long, default_value = "127.0.0.1")]
	host: String,
}

fn main() {
	let cli = Cli::parse();
	let addr = IpAddr::from_str(cli.host.as_str()).unwrap();
	let mut socket = TcpStream::connect((addr, cli.port)).unwrap();
	socket.set_nonblocking(true).unwrap();
	let mut listenner = socket.try_clone().unwrap();
	thread::scope(|scope| {
		let running = Arc::new(Mutex::new(true));
		let run = running.clone();
		scope.spawn(move || {
			let mut data = Vec::new();
			while *run.lock().unwrap() {
				if let Ok(_) = listenner.read_to_end(&mut data) {
					println!("{:?}", String::from_utf8_lossy(&data));
				}
			}
		});

		let mut stdout = stdout();
		let stdin = stdin();
		stdout.write("Exit command: 'exit'\n".as_bytes()).unwrap();
		while *running.lock().unwrap() {
			let mut cmd = String::new();
			stdout.write(" > ".as_bytes()).unwrap();
			stdout.flush().unwrap();
			let res = stdin.read_line(&mut cmd);
			if res.is_err() || cmd.trim().to_lowercase() == "exit" {
				if let Ok(mut lock) = running.lock() {
					*lock = false;
				} else {
					panic!("Could not lock on the running");
				}
			} else if cmd.len() > 0 {
				let cmd = cmd.trim();
				socket.write_all(&(cmd.len() as u32).to_be_bytes()).unwrap();
				socket.write_all(cmd.as_bytes()).unwrap();
			}
		}
		socket.shutdown(Shutdown::Both).unwrap();
	});
}