use serde::{Deserialize, Serialize};
use interfaces::models::licence::DsmLicense;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct License {
	pub(crate) id: u32,
	pub(crate) name: String,
	pub(crate) url: String,
}

impl License {
	pub(crate) fn from_dsm(id: &u32, license: &DsmLicense) -> Self {
		Self {
			id: *id,
			name: license.name.clone(),
			url: license.url.clone(),
		}
	}
	
	pub fn to_dsm(&self) -> DsmLicense {
		DsmLicense::new(self.name.clone(), self.url.clone())
	}
}