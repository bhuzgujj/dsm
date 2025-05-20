mod obj_data;
mod data_entries;
mod obj_names;

use crate::yolo_1_1::obj_data::ObjData;
use interfaces::models::metadata::DsmMetaData;
use interfaces::models::DataForm;
use interfaces::models::DsmSets;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::vec;

const PREFIX: &str = "data/";

pub(crate) fn read(root: &PathBuf, name: Option<String>, version: u32, formatter: DataForm) -> anyhow::Result<Vec<DsmSets>> {
	let obj_data = ObjData::read(root)?;
	let classes = obj_names::read(root.join(obj_data.get_names()))?;
	let mut sets = HashMap::new();
	let new_name = name.clone().unwrap_or(
		root.file_name()
			.expect("Could not get the directory name")
			.to_string_lossy()
			.into(),
	);
	for (key, value) in obj_data.get_sets() {
		let entries = data_entries::read(&root.join(value), &new_name, version, root, &classes)?;
		if sets.contains_key(key) {
			return Err(anyhow::anyhow!("duplicated set: {}", key))
		}
		sets.insert(key.clone(), entries);
	}
	let metadata = DsmMetaData::new(
		new_name,
		version,
		None,
		String::new(),
		String::new(),
		String::new(),
		String::new(),
		String::new(),
		formatter,
		classes,
		HashMap::new()
	);
	Ok(vec![DsmSets::new(metadata, sets)])
}

pub(crate) fn write(store_path: &PathBuf, root: &PathBuf, datasets: &DsmSets) -> anyhow::Result<()> {
	create_dir_all(root)?;
	obj_names::write(root, datasets.get_classes())?;
	let mut sets = HashMap::new();
	for (key, value) in datasets.get_entries() {
		sets.insert(key.clone(), data_entries::write(
			store_path,
			root,
			key,
			value
		)?);
	}
	ObjData::new(datasets.get_classes().len() as u32, sets).write(root)?;
	Ok(())
}

pub(crate) fn strip_prefix(value: &str) -> String {
	if value.starts_with(PREFIX) {
		value[PREFIX.len()..].to_string()
	} else { value.to_string() }
}