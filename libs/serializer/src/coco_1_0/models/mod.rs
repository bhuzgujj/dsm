use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use crate::coco_1_0::models::sequence::Sequence;

pub mod info;
pub mod license;
pub mod sequence;
pub mod category;
pub mod image;
pub mod annotation;
pub mod attribute;

pub(crate) fn read(root: &PathBuf) -> anyhow::Result<HashMap<String, Sequence>> {
	let mut sequences = HashMap::new();
	for files in read_dir(root)? {
		let f = files?;
		if let Ok(content) = read_to_string(f.path()) {
			sequences.insert(
				f.file_name().to_string_lossy().to_string(),
				serde_json::from_str(content.as_str())?
			);
		}

	}
	Ok(sequences)
}