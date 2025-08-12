use interfaces::models::datasets::Classes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CocoCategory {
	pub(crate) id: u32,
	pub(crate) name: String,
	pub(crate) supercategory: String,
}

impl CocoCategory {
	pub fn from_dsm(classes: &Classes, id: u32) -> Self {
		Self {
			id: id + 1,
			name: classes.class().clone(),
			supercategory: classes.subclass().clone().unwrap_or_default(),
		}
	}

	pub fn dsm(&self) -> Classes {
		Classes::new(self.name.clone(), Some(self.supercategory.clone()))
	}
}
