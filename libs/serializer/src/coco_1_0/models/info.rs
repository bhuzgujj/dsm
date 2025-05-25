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
	pub(crate) fn from_dsm(meta: &DsmMetaData) -> Self {
		Self {
			contributor: meta.get_contributor().clone(),
			date_created: meta.get_date_created().clone(),
			description: meta.get_description().clone(),
			url: meta.get_url().clone(),
			version: meta.get_url().clone(),
			year: meta.get_year().clone()
		}
	}
}