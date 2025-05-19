use serde::{Deserialize, Serialize};
use interfaces::models::classes::Classes;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Category {
	pub(crate) id: u32,
	pub(crate) name: String,
	pub(crate) supercategory: String
}

impl Category {
	pub fn from_classes(classes: &Classes, id: u32) -> Self {
		Self {
			id,
			name: classes.get_classes().clone(),
			supercategory: classes.get_subclass().clone().unwrap_or(String::new()),
		}
	}

	pub fn to_classes(&self) -> Classes {
		Classes::new(self.name.clone(), Some(self.supercategory.clone()))
	}
}