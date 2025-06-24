use std::fs::{copy, create_dir_all};
use std::path::Path;
use interfaces::log_err;
use interfaces::models::DsmSets;
use interfaces::models::DsmDataForm;
use interfaces::paths::write_to_file;
use crate::coco_1_0::models::sequence::{Sequence, IMAGE_PATH};

mod models;

const ANNOTATION_DIR: &str = "annotations";

pub(crate) fn read(root: &Path, name: Option<String>, version: String, data_form: DsmDataForm) -> anyhow::Result<Vec<DsmSets>> {
	let sequences = models::read(&root.join(ANNOTATION_DIR))?;
	let mut datasets = Vec::new();
	let new_name = name.clone().unwrap_or(
	    root.file_name()
	        .expect("Could not get the directory name")
	        .to_string_lossy()
	        .into(),
	);
	for (json, sequence) in sequences {
		datasets.push(sequence.dsm(&data_form, &new_name, &version, json)?);
	}
	Ok(datasets)
}

pub(crate) fn write(input: &Path, output: &Path, datasets: &DsmSets) -> anyhow::Result<()> {
	if let Err(err) = create_dir_all(output.join(ANNOTATION_DIR)) {
		return log_err!(format!("Failed to create dir {}: {}", output.join(ANNOTATION_DIR).display(), err))
	}
	let (sequences, image_map) = Sequence::from_dsm(datasets);
	for (name, sequence) in sequences {
		let json = match serde_json::to_string_pretty(&sequence) {
			Ok(ctnt) => ctnt,
			Err(err) => return log_err!(format!("Could not serialize Sequence '{}': {}", output.join(IMAGE_PATH).join(&name).display(), err))
		};
		write_to_file(
			&output.join(ANNOTATION_DIR).join(format!("instances_{}.json", name)),
			json,
			true,
			true,
		)?;
		if let Err(err) = create_dir_all(output.join(IMAGE_PATH).join(&name)) {
			return log_err!(format!("Failed to create dir {}: {}", output.join(IMAGE_PATH).join(&name).display(), err))
		}
		for image in sequence.images {
			if let Some(img) = image_map.get(&image.file_name) {
				if let Err(err) = copy(
					input.join(img),
					output.join(IMAGE_PATH).join(&name).join(&image.file_name)
				) {
					return log_err!(format!(
						"Failed to copy '{}' to '{}': {}",
						input.join(img).display(),
						output.join(IMAGE_PATH).join(&name).join(image.file_name.clone()).display(),
						err
					))
				}
			} else {
				return log_err!(format!("Image does not exist: {}", image.file_name))
			}
		}
	}

	Ok(())
}