mod data_entries;
mod obj_data;
mod obj_names;

use crate::yolo_1_1::obj_data::ObjData;
use interfaces::models::datasets::Dataset;
use interfaces::models::datasets::MetaDataBuilder;
use interfaces::models::DataFormat;
use log::debug;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::Path;

const PREFIX: &str = "data/";

#[cfg(test)]
pub const FORMATTER: DataFormat = DataFormat::Yolo1_1;

pub(crate) fn read(
	root: &Path,
	name: Option<String>,
	version: String,
	formatter: DataFormat,
) -> anyhow::Result<Dataset> {
	debug!("Read in yolo format");
	let obj_data = ObjData::read(root)?;
	let classes = obj_names::read(root.join(obj_data.names()))?;
	let mut sets = HashMap::new();
	let new_name = name.clone().unwrap_or(
		root.file_name()
			.expect("Could not get the directory name")
			.to_string_lossy()
			.into(),
	);
	for (key, value) in obj_data.sets() {
		let entries = data_entries::read(&root.join(value), root, &classes)?;
		if sets.contains_key(key) {
			return Err(anyhow::anyhow!("duplicated set: {}", key));
		}
		sets.insert(key.clone(), entries);
	}
	let metadata = MetaDataBuilder::new(new_name, version, formatter, classes).build();
	Ok(Dataset::new(metadata, sets))
}

pub(crate) fn write(output: &Path, datasets: &Dataset) -> anyhow::Result<()> {
	debug!("Write in yolo format");
	create_dir_all(output)?;
	obj_names::write(output, datasets.classes().clone())?;
	let mut sets = HashMap::new();
	for (key, value) in datasets.entries() {
		sets.insert(key.clone(), data_entries::write(output, key, value)?);
	}
	ObjData::new(datasets.classes().len() as u32, sets).write(output)?;
	Ok(())
}

pub(crate) fn strip_prefix(value: &str) -> String {
	if let Some(stripped) = value.strip_prefix(PREFIX) {
		stripped.to_string()
	} else {
		value.to_string()
	}
}
