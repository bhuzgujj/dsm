use crate::yolo_1_1::strip_prefix;
use image::image_dimensions;
use interfaces::models::annotation::DsmAnnotation;
use interfaces::models::classes::DsmClasses;
use interfaces::models::entries::DsmEntry;
use std::collections::HashMap;
use std::fs::{copy, create_dir_all, read_to_string};
use std::path::PathBuf;
use std::str::FromStr;
use interfaces::logger::error;
use interfaces::paths::write_to_file;

pub(crate) fn read(refs: &PathBuf, datasets_name: &String, datasets_version: &String, root: &PathBuf, classes: &HashMap<u32, DsmClasses>) -> anyhow::Result<Vec<DsmEntry>> {
	let content = match read_to_string(refs) {
		Ok(ctnt) => ctnt,
		Err(err) => return error(format!("Could not read '{}': {}", refs.display(), err))
	};
	let version = format!("v{}", datasets_version);
	let mut entries = Vec::new();
	for line in content.lines() {
		let image_relative_path = strip_prefix(line.trim());
		let images_path = root.join(&image_relative_path);
		let (img_width, img_height) = match image_dimensions(&images_path) {
			Ok((w, h)) => (w, h),
			Err(err) => return error(format!("Could not read image '{}': {}", images_path.display(), err))
		};
		let mut annotation_path = images_path.clone();
		annotation_path.set_extension("txt");
		let annotations_file = match read_to_string(annotation_path) {
			Ok(ctnt) => ctnt,
			Err(err) => return error(format!("Could not read '{}': {}", refs.display(), err))
		};
		let mut annotations = Vec::new();
		for annotation in annotations_file.lines() {
			let annotation = annotation.trim();
			if !annotation.is_empty() {
				annotations.push(match parse_annotation(annotation, classes, img_width, img_height) {
					Ok(annot) => annot,
					Err(err) => return error(format!("Could not parse annotation '{}': {}", annotation, err))
				});
			}
		}
		entries.push(DsmEntry::new(
			PathBuf::from(&datasets_name).join(&version).join(image_relative_path),
			img_width,
			img_height,
			images_path.file_name().unwrap().to_str().unwrap().to_string(),
			None,
			None,
			None,
			None,
			annotations
		));
	}
	Ok(entries)
}

fn parse_annotation(entry: &str, classes: &HashMap<u32, DsmClasses>, img_width: u32, img_height: u32) -> anyhow::Result<DsmAnnotation> {
	let fields: Vec<&str> = entry.split(&['\t', ' ']).collect();
	if fields.len() != 5 {
		return Err(anyhow::anyhow!("Invalid annotation, missing fields: '{}'", entry));
	}
	let class_id = u32::from_str(fields[0])?;
	let _ = classes.get(&class_id).ok_or_else(|| anyhow::anyhow!("Invalid annotation id: {}", class_id))?;

	let x = f64::from_str(fields[1])? * img_width as f64;
	let y = f64::from_str(fields[2])? * img_height as f64;
	let width = f64::from_str(fields[3])? * img_width as f64;
	let height = f64::from_str(fields[4])? * img_height as f64;
	Ok(DsmAnnotation::new(
		class_id,
		x,
		y,
		width,
		height,
		Vec::new(),
		0,
		false,
		0
	))
}

pub(crate) fn write(store_path: &PathBuf, root: &PathBuf, sets_name: &String, dataset_entry: &Vec<DsmEntry>) -> anyhow::Result<String> {
	let dir = format!("obj_{sets_name}_data");
	let img_dir=  root.join(&dir);
	if let Err(err) = create_dir_all(&img_dir) {
		return error(format!("Failed to create dir {}: {}", img_dir.display(), err))
	}
	let mut sets = Vec::new();
	let data_dir = store_path;
	for entry in dataset_entry {
		let new_image_name = entry.get_file_name();
		sets.push(format!("{}/{}", dir.clone(), new_image_name.clone()));
		if let Err(err) = copy(
			data_dir.join(entry.get_image_relative_path()),
			img_dir.join(new_image_name)
		) {
			return error(format!(
				"Failed to copy '{}' to '{}': {}",
				data_dir.join(entry.get_image_relative_path()).display(),
				img_dir.join(new_image_name).display(),
				err
			))
		}

		let (img_width, img_height) = match image_dimensions(img_dir.join(new_image_name)) {
			Ok((w, h)) => (w, h),
			Err(err) => return error(format!("Could not read image '{}': {}", img_dir.join(new_image_name).display(), err))
		};

		let mut annotation_file = PathBuf::from(entry.get_file_name().as_str());
		annotation_file.set_extension("txt");
		let new_annotation_name = annotation_file
			.file_name()
			.expect("Invalid")
			.to_str()
			.expect("Invalid");
		write_to_file(
			&img_dir.join(new_annotation_name),
			entry.get_annotation().iter()
				.map(|a| a.to_file_percent_str(img_width, img_height))
				.collect::<Vec<String>>()
				.join("\n"),
			true,
			true
		)?;
	}
	let set_file = format!("{sets_name}.txt");
	write_to_file(&root.join(&set_file), sets.join("\n"), true, true)?;
	Ok(set_file)
}