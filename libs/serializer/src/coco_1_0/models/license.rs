use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct License {
	pub(crate) id: u32,
	pub(crate) name: String,
	pub(crate) url: String,
}

impl License {
	pub fn to_datasettable(&self) -> interfaces::models::licence::License {
		interfaces::models::licence::License::new(self.name.clone(), self.url.clone())
	}
}