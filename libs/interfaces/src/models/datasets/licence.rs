use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct License {
	pub name: String,
	pub url: String,
}

impl License {
	pub fn new(name: String, url: String) -> Self {
		Self { name, url }
	}
}