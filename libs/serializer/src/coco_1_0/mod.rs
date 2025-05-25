use std::fs::{copy, create_dir_all};
use std::path::PathBuf;
use interfaces::logger::error;
use interfaces::models::DsmSets;
use interfaces::models::DsmDataForm;
use interfaces::paths::write_to_file;
use crate::coco_1_0::models::sequence::{Sequence, IMAGE_PATH};

mod models;

const ANNOTATION_DIR: &str = "annotations";

pub(crate) fn read(root: &PathBuf, name: Option<String>, version: String, data_form: DsmDataForm) -> anyhow::Result<Vec<DsmSets>> {
	let sequences = models::read(&root.join(ANNOTATION_DIR))?;
	let mut datasets = Vec::new();
	let new_name = name.clone().unwrap_or(
	    root.file_name()
	        .expect("Could not get the directory name")
	        .to_string_lossy()
	        .into(),
	);
	for (json, sequence) in sequences {
		datasets.push(sequence.to_dsm(&data_form, &new_name, &version, json)?);
	}
	Ok(datasets)
}

pub(crate) fn write(store_path: &PathBuf, root: &PathBuf, datasets: &DsmSets) -> anyhow::Result<()> {
	if let Err(err) = create_dir_all(root.join(ANNOTATION_DIR)) {
		return error(format!("Failed to create dir {}: {}", root.join(ANNOTATION_DIR).display(), err))
	}
	let (sequences, image_map) = Sequence::from_dsm(datasets);
	for (name, sequence) in sequences {
		let json = match serde_json::to_string_pretty(&sequence) {
			Ok(ctnt) => ctnt,
			Err(err) => return error(format!("Could not serialize Sequence '{}': {}", root.join(IMAGE_PATH).join(&name).display(), err))
		};
		write_to_file(
			&root.join(ANNOTATION_DIR).join(format!("instances_{}.json", name)),
			json,
			true,
			true,
		)?;
		if let Err(err) = create_dir_all(root.join(IMAGE_PATH).join(&name)) {
			return error(format!("Failed to create dir {}: {}", root.join(IMAGE_PATH).join(&name).display(), err))
		}
		for image in sequence.images {
			if let Some(img) = image_map.get(&image.file_name) {
				if let Err(err) = copy(
					store_path.join(img),
					root.join(IMAGE_PATH).join(&name).join(&image.file_name)
				) {
					return error(format!(
						"Failed to copy '{}' to '{}': {}",
						store_path.join(img).display(),
						root.join(IMAGE_PATH).join(&name).join(image.file_name).display(),
						err
					))
				}
			} else {
				return error(format!("Image does not exist: {}", image.file_name))
			}
		}
	}

	Ok(())
}