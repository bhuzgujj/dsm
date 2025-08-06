use interfaces::models::classes::DsmClasses;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Category {
	pub(crate) id: u32,
	pub(crate) name: String,
	pub(crate) supercategory: String,
}

impl Category {
	pub fn from_dsm(classes: &DsmClasses, id: u32) -> Self {
		Self {
			id: id + 1,
			name: classes.class().clone(),
			supercategory: classes.subclass().clone().unwrap_or_default(),
		}
	}

	pub fn dsm(&self) -> DsmClasses {
		DsmClasses::new(self.name.clone(), Some(self.supercategory.clone()))
	}
}
