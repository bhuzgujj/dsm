use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct License {
	pub(crate) id: u32,
	pub(crate) name: String,
	pub(crate) url: String,
}

impl License {
	pub(crate) fn from_base(id: &u32, license: &interfaces::models::licence::DsmLicense) -> Self {
		Self {
			id: id.clone(),
			name: license.name.clone(),
			url: license.url.clone(),
		}
	}
}

impl License {
	pub fn to_datasettable(&self) -> interfaces::models::licence::DsmLicense {
		interfaces::models::licence::DsmLicense::new(self.name.clone(), self.url.clone())
	}
}