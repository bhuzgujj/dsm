use serde::{Deserialize, Serialize};
use interfaces::models::metadata::DsmMetaData;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Info {
	pub(crate) contributor: String,
	pub(crate) date_created: String,
	pub(crate) description: String,
	pub(crate) url: String,
	pub(crate) version: String,
	pub(crate) year: String
}

impl Info {
	pub(crate) fn from_meta(meta: &DsmMetaData) -> Self {
		Self {
			contributor: meta.get_contributor(),
			date_created: meta.get_date_created(),
			description: meta.get_description(),
			url: meta.get_url(),
			version: meta.get_url(),
			year: meta.get_year()
		}
	}
}