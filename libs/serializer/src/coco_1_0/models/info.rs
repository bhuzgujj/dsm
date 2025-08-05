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
			contributor: meta.contributor().clone(),
			date_created: meta.date_created().clone(),
			description: meta.description().clone(),
			url: meta.url().clone(),
			version: meta.url().clone(),
			year: meta.year().clone()
		}
	}
}