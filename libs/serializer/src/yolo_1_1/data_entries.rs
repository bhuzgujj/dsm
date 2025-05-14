use std::collections::HashMap;
use std::fs::read_to_string;
use std::hash::Hash;
use std::path::PathBuf;
use std::str::FromStr;
use image::image_dimensions;
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
	let class = classes.get(&class_id).ok_or_else(|| anyhow::anyhow!("Invalid annotation id: {}", class_id))?;

	let x = f64::from_str(fields[1])?;
	let y = f64::from_str(fields[2])?;
	let width = f64::from_str(fields[3])?;
	let height = f64::from_str(fields[4])?;
	Ok(Annotation::new(class, x, y, width, height))
}