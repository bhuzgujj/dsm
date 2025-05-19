use std::fs::{copy, create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use interfaces::models::Datasets;
use interfaces::models::DataForm;
use crate::coco_1_0::models::sequence::{Sequence, IMAGE_PATH};

mod models;

const ANNOTATION_DIR: &'static str = "annotations";

pub(crate) fn read(root: &PathBuf, name: Option<String>, version: Option<u32>, data_form: DataForm) -> anyhow::Result<Vec<Datasets>> {
	let sequences = models::read(&root.join(ANNOTATION_DIR))?;
	let mut datasets = Vec::new();
	let new_name = name.clone().unwrap_or(
	    root.file_name()
	        .expect("Could not get the directory name")
	        .to_string_lossy()
	        .into(),
	);
	for (json, sequence) in sequences {
		datasets.push(sequence.to_datasets(&data_form, &new_name, &version, json)?);
	}
	Ok(datasets)
}

pub(crate) fn write(store_path: &PathBuf, root: &PathBuf, datasets: &Datasets) -> anyhow::Result<()> {
	create_dir_all(&root.join(ANNOTATION_DIR))?;
	let (sequences, image_map) = Sequence::from_datasets(datasets)?;
	for (name, sequence) in sequences {
		let json = serde_json::to_string_pretty(&sequence)?;
		OpenOptions::new()
			.write(true)
			.truncate(true)
			.open(&root.join(ANNOTATION_DIR).join(format!("instances_{}.json", name)))?
			.write_all(json.as_bytes())?;
		create_dir_all(&root.join(IMAGE_PATH).join(&name))?;
		for image in sequence.images {
			copy(
				&store_path.join(image_map.get(&image.file_name).expect("TODO: Image not found")),
				&root.join(IMAGE_PATH).join(&name).join(image.file_name)
			)?;
		}
	}

	Ok(())
}