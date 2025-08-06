use crate::coco_1_0::models::sequence::{Sequence, IMAGE_PATH};
use interfaces::log_err;
use interfaces::models::DsmDataForm;
use interfaces::models::DsmSets;
use interfaces::paths::write_to_file;
use std::fs::{copy, create_dir_all};
use std::path::Path;

mod models;

const ANNOTATION_DIR: &str = "annotations";

pub(crate) fn read(
	root: &Path,
	name: Option<String>,
	version: String,
	data_form: DsmDataForm,
) -> anyhow::Result<Vec<DsmSets>> {
	let sequences = models::read(&root.join(ANNOTATION_DIR))?;
	let mut datasets = Vec::new();
	let new_name = name.clone().unwrap_or(
		root.file_name()
			.expect("Could not get the directory name")
			.to_string_lossy()
			.into(),
	);
	for (json, sequence) in sequences {
		datasets.push(sequence.dsm(&data_form, &new_name, &version, json, root)?);
	}
	Ok(datasets)
}

pub(crate) fn write(output: &Path, datasets: &DsmSets) -> anyhow::Result<()> {
	if let Err(err) = create_dir_all(output.join(ANNOTATION_DIR)) {
		return log_err!(format!(
			"Failed to create dir {}: {}",
			output.join(ANNOTATION_DIR).display(),
			err
		));
	}
	let sequences = Sequence::from_dsm(datasets);
	for (name, sequence) in sequences {
		let json = match serde_json::to_string_pretty(&sequence) {
			Ok(ctnt) => ctnt,
			Err(err) => {
				return log_err!(format!(
					"Could not serialize Sequence '{}': {}",
					output.join(IMAGE_PATH).join(&name).display(),
					err
				));
			},
		};
		write_to_file(
			&output
				.join(ANNOTATION_DIR)
				.join(format!("instances_{name}.json")),
			json,
			true,
			true,
		)?;
		if let Err(err) = create_dir_all(output.join(IMAGE_PATH).join(&name)) {
			return log_err!(format!(
				"Failed to create dir {}: {}",
				output.join(IMAGE_PATH).join(&name).display(),
				err
			));
		}

		for image in sequence.images {
			for entries in datasets.entries().values() {
				for entry in entries {
					if image.is_entry(entry) {
						let image_path = entry.image_location();
						if let Err(err) = copy(
							&image_path,
							output.join(IMAGE_PATH).join(&name).join(&image.file_name),
						) {
							return log_err!(format!(
								"Failed to copy '{}' to '{}': {}",
								image_path.to_string_lossy().to_string(),
								output
									.join(IMAGE_PATH)
									.join(&name)
									.join(image.file_name.clone())
									.display(),
								err
							));
						}

						break;
					}
				}
			}
		}
	}

	Ok(())
}
