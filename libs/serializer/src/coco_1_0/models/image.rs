use interfaces::models::entries::DsmEntry;
use serde::{Deserialize, Deserializer, Serialize};
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
	pub(crate) date_captured: String,
}

impl Image {
	pub(crate) fn from_dsm(id: u32, data_entry: &DsmEntry) -> Self {
		Self {
			id,
			width: *data_entry.width(),
			height: *data_entry.height(),
			file_name: data_entry.file_name().clone(),
			license: data_entry.license().unwrap_or(0),
			flickr_url: data_entry.flickr_url().clone(),
			coco_url: data_entry.coco_url().clone(),
			date_captured: data_entry
				.date_captured()
				.clone()
				.unwrap_or(String::from("")),
		}
	}

	pub(crate) fn is_entry(&self, entry: &DsmEntry) -> bool {
		self.file_name.eq(entry.file_name())
			&& (entry.original_id().is_none()
				|| entry
					.original_id()
					.as_ref()
					.is_some_and(|og_id| og_id.eq(&self.id.to_string())))
	}
}

fn number_and_string_deserializer<'de, D>(deserializer: D) -> Result<String, D::Error>
where
	D: Deserializer<'de>,
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
