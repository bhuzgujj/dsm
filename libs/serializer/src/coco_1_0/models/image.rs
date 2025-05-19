use serde::{Deserialize, Serialize};

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