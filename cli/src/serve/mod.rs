mod session;

use std::{
    io::stdin,
    net::{IpAddr, TcpListener},
    str::FromStr,
    thread::{self},
};

use clap::Args;
use interfaces::models::Settings;
use log::error;

use crate::serve::session::Session;

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
        let mut running = true;
        let addr = IpAddr::from_str(self.host.as_str())?;
        let socket = TcpListener::bind((addr, self.port))?;
        thread::scope(|scope| {
            scope.spawn(|| {
                while running {
                    match socket.accept() {
                        Ok((stream, from)) => {
                            let mut session = Session::new(stream, from);
                            scope.spawn(move || {
                                session.process(settings);
                                session.stop();
                            });
                        }
                        Err(err) => error!("{err}"),
                    }
                }
            });
        });
        println!("Server is running... (exit command: 'exit')");
        let cli = stdin();
        while running {
            let mut cmd = String::new();
            let res = cli.read_line(&mut cmd);
            if res.is_err() || cmd.to_lowercase() == "exit" {
                running = false;
            }
        }
        Ok(())
    }
}
