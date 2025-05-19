use serde::{Deserialize, Serialize};
use interfaces::models::classes::DsmClasses;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Category {
	pub(crate) id: u32,
	pub(crate) name: String,
	pub(crate) supercategory: String
}

impl Category {
	pub fn from_classes(classes: &DsmClasses, id: u32) -> Self {
		Self {
			id: id + 1,
			name: classes.get_classes().clone(),
			supercategory: classes.get_subclass().clone().unwrap_or_default(),
		}
	}

	pub fn to_classes(&self) -> DsmClasses {
		DsmClasses::new(self.name.clone(), Some(self.supercategory.clone()))
	}
}