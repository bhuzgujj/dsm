use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CocoLicense {
	pub(crate) id: u32,
	pub(crate) name: String,
	pub(crate) url: String,
}

impl CocoLicense {
	pub(crate) fn from_dsm(id: &u32, license: &interfaces::models::datasets::License) -> Self {
		Self {
			id: *id,
			name: license.name().to_string(),
			url: license.url().to_string(),
		}
	}

	pub fn dsm(&self) -> interfaces::models::datasets::License {
		interfaces::models::datasets::License::new(self.name.clone(), self.url.clone())
	}
}
