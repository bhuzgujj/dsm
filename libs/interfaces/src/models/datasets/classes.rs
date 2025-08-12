use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Classes {
	class: String,
	subclass: Option<String>,
}

impl Classes {
	pub fn new(class: String, subclass: Option<String>) -> Self {
		Self { class, subclass }
	}

	pub fn class(&self) -> &String {
		&self.class
	}
	pub fn subclass(&self) -> &Option<String> {
		&self.subclass
	}
}
