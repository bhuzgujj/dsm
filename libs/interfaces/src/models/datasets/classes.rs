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
	
	pub fn get_classes(&self) -> &String {
		&self.class
	}
	pub fn get_subclass(&self) -> &Option<String> {
		&self.subclass
	}
}