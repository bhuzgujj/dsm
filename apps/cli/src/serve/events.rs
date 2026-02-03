use std::net::SocketAddr;

pub enum ClientEvent {
	Close
}

pub enum ServerEvent {
	Close,
	CloseClient {
		addr: SocketAddr,
	}
}