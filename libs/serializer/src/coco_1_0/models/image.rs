use serde::{Deserialize, Serialize};
use interfaces::models::entries::DsmEntry;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Image {
	pub(crate) id: u32,
	pub(crate) width: u32,
	pub(crate) height: u32,
	pub(crate) file_name: String,
	pub(crate) license: u32,
	pub(crate) flickr_url: String,
	pub(crate) coco_url: String,
	pub(crate) date_captured: u32
}

impl Image {
	pub(crate) fn from_data_entry(id: u32, data_entry: &DsmEntry) -> Self {
		Self {
			id,
			width: data_entry.get_width(),
			height: data_entry.get_height(),
			file_name: data_entry.get_file_name(),
			license: data_entry.get_license().unwrap_or(0),
			flickr_url: data_entry.get_flickr_url().clone().unwrap_or_default(),
			coco_url: data_entry.get_coco_url().clone().unwrap_or_default(),
			date_captured: data_entry.get_date_captured().unwrap_or(0)
		}
	}
}