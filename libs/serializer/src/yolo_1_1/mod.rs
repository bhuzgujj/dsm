mod obj_data;
mod data_entries;
mod obj_names;

use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::PathBuf;
use serde_json::Value::Object;
use interfaces::models::Datasets;
use interfaces::models::metadata::MetaData;
use crate::yolo_1_1::obj_data::ObjData;

const PREFIX: &str = "data/";

pub(crate) fn read(root: &PathBuf, name: String, version: u32) -> anyhow::Result<Datasets> {
	let objdata = ObjData::read(root)?;
	let classes = obj_names::read(root.join(objdata.get_names()))?;
	let mut sets = HashMap::new();
	for (key, value) in objdata.get_sets() {
		let entries = data_entries::read(&root.join(value), &root, &classes)?;
		if sets.contains_key(key) {
			return Err(anyhow::anyhow!("duplicated set: {}", key))
		}
		sets.insert(key.clone(), entries);
	}
	let metadata = MetaData::new(name, version, classes);
	Ok(Datasets::new(metadata, sets))
}

pub(crate) fn write(store_path: &PathBuf, root: &PathBuf, datasets: &Datasets) -> anyhow::Result<()> {
	create_dir_all(&root)?;
	obj_names::write(&root, datasets.get_classes())?;
	let dataset_prefix = format!("{}-v{}-", datasets.get_name(), datasets.get_version());
	let mut sets = HashMap::new();
	for (key, value) in datasets.get_entries() {
		sets.insert(key.clone(), data_entries::write(
			&store_path,
			&root,
			&dataset_prefix,
			key,
			value
		)?);
	}
	ObjData::new(datasets.get_classes().len() as u32, sets).write(&root)?;
	Ok(())
}

pub(crate) fn strip_prefix(value: &str) -> String {
	if value.starts_with(PREFIX) {
		value[PREFIX.len()..].to_string()
	} else { value.to_string() }
}