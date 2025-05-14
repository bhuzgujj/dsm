use std::collections::HashMap;
use std::fs::{copy, create_dir_all, read_to_string, OpenOptions};
use std::hash::Hash;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use image::image_dimensions;
use serde::de::Unexpected::Option;
use interfaces::models::annotation::Annotation;
use interfaces::models::entries::DatasetEntry;
use crate::yolo_1_1::strip_prefix;

pub(crate) fn read(refs: &PathBuf, root: &PathBuf, classes: &HashMap<u32, String>) -> anyhow::Result<Vec<DatasetEntry>> {
	let content = read_to_string(refs)?;
	let mut entries = Vec::new();
	for line in content.lines() {
		let images_path = root.join(strip_prefix(line.trim()));
		let mut annotation_path = images_path.clone();
		annotation_path.set_extension("txt");
		let annotations_file = read_to_string(&annotation_path)?;
		let mut annotations = Vec::new();
		for annotation in annotations_file.lines() {
			let annotation = annotation.trim();
			if annotation.len() > 0 {
				annotations.push(parse_annotation(&annotation, &classes)?);
			}
		}
		let (width, height) = image_dimensions(&images_path)?;
		entries.push(DatasetEntry::new(images_path.clone(), width, height, annotations));
	}
	Ok(entries)
}

fn parse_annotation(entry: &str, classes: &HashMap<u32, String>) -> anyhow::Result<Annotation> {
	let fields: Vec<&str> = entry.split(&['\t', ' ']).collect();
	if fields.len() != 5 {
		return Err(anyhow::anyhow!("Invalid annotation, missing fields: '{}'", entry));
	}
	let class_id = u32::from_str(fields[0])?;
	let _ = classes.get(&class_id).ok_or_else(|| anyhow::anyhow!("Invalid annotation id: {}", class_id))?;

	let x = f64::from_str(fields[1])?;
	let y = f64::from_str(fields[2])?;
	let width = f64::from_str(fields[3])?;
	let height = f64::from_str(fields[4])?;
	Ok(Annotation::new(class_id, x, y, width, height))
}

pub(crate) fn write(store_path: &PathBuf, root: &PathBuf, dataset_prefix: &String, sets_name: &String, dataset_entry: &Vec<DatasetEntry>) -> anyhow::Result<String> {
	let dir = format!("obj_{sets_name}_data");
	let img_dir=  root.join(&dir);
	create_dir_all(&img_dir)?;
	let mut sets = Vec::new();
	for entry in dataset_entry {
		let new_image_name = format!("{dataset_prefix}{}", entry.get_image_path()
			.file_name()
			.expect("Invalid")
			.to_str()
			.expect("Invalid")
		);
		sets.push(format!("{}/{}", dir.clone(), new_image_name.clone()));
		copy(
			&store_path.join(entry.get_image_path()),
		    &img_dir.join(new_image_name)
		)?;

		let mut annotation_file = entry.get_image_path();
		annotation_file.set_extension("txt");
		let new_annotation_name = format!("{dataset_prefix}{}", annotation_file
			.file_name()
			.expect("Invalid")
			.to_str()
			.expect("Invalid")
		);
		OpenOptions::new()
			.write(true)
			.truncate(true)
			.create(true)
			.open(img_dir.join(&new_annotation_name))?
			.write_all(entry.get_annotation().iter()
				.map(|a| a.to_file_str())
				.collect::<Vec<String>>()
				.join("\n").as_bytes())?;
	}
	let set_file = format!("{sets_name}.txt");
	OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(root.join(&set_file))?
		.write_all(sets.join("\n").as_bytes())?;
	Ok(set_file)
}