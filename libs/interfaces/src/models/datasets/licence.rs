use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct License {
	name: String,
	url: String,
}

impl License {
	pub fn new(name: String, url: String) -> Self {
		Self { name, url }
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn url(&self) -> &str {
		&self.url
	}
}
