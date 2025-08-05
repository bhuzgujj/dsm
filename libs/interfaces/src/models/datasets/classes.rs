use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DsmClasses {
	class: String,
	subclass: Option<String>,
}

impl DsmClasses {
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