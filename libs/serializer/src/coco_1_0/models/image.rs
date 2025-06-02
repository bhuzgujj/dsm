use serde::{Deserialize, Deserializer, Serialize};
use interfaces::models::entries::DsmEntry;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Image {
	pub(crate) id: u32,
	pub(crate) width: u32,
	pub(crate) height: u32,
	pub(crate) file_name: String,
	pub(crate) license: u32,
	pub(crate) flickr_url: Option<String>,
	pub(crate) coco_url: Option<String>,
	#[serde(deserialize_with = "number_and_string_deserializer")]
	pub(crate) date_captured: String
}

impl Image {
	pub(crate) fn from_dsm(id: u32, data_entry: &DsmEntry) -> Self {
		Self {
			id,
			width: *data_entry.get_width(),
			height: *data_entry.get_height(),
			file_name: data_entry.get_file_name().clone(),
			license: data_entry.get_license().unwrap_or(0),
			flickr_url: data_entry.get_flickr_url().clone(),
			coco_url: data_entry.get_coco_url().clone(),
			date_captured: data_entry.get_date_captured().clone().unwrap_or(String::from(""))
		}
	}
}

fn number_and_string_deserializer<'de, D>(deserializer: D) -> Result<String, D::Error>
where 
	D: Deserializer<'de>
{
	let raw: Value = Deserialize::deserialize(deserializer)?;
	match raw {
		Value::Null => todo!("TODO: find a way to Err here"),
		Value::Bool(_) => todo!("TODO: find a way to Err here"),
		Value::Number(number) => Ok(number.to_string()),
		Value::String(string) => Ok(string.to_string()),
		Value::Array(_) => todo!("TODO: find a way to Err here"),
		Value::Object(_) => todo!("TODO: find a way to Err here"),
	}
}