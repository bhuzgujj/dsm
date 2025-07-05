use crate::coco_1_0::models::sequence::{Sequence, IMAGE_PATH};
use interfaces::log_err;
use interfaces::models::DsmDataForm;
use interfaces::models::DsmSets;
use interfaces::paths::write_to_file;
use std::collections::HashMap;
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
        datasets.push(sequence.dsm(&data_form, &new_name, &version, json)?);
    }
    Ok(datasets)
}

pub(crate) fn write(
    input: &Path,
    output: &Path,
    datasets: &DsmSets,
    image_rel_path: &Option<HashMap<String, String>>,
) -> anyhow::Result<()> {
    if let Err(err) = create_dir_all(output.join(ANNOTATION_DIR)) {
        return log_err!(format!(
            "Failed to create dir {}: {}",
            output.join(ANNOTATION_DIR).display(),
            err
        ));
    }
    let (sequences, image_map) = Sequence::from_dsm(datasets);
    for (name, sequence) in sequences {
        let json = match serde_json::to_string_pretty(&sequence) {
            Ok(ctnt) => ctnt,
            Err(err) => {
                return log_err!(format!(
                    "Could not serialize Sequence '{}': {}",
                    output.join(IMAGE_PATH).join(&name).display(),
                    err
                ))
            }
        };
        write_to_file(
            &output
                .join(ANNOTATION_DIR)
                .join(format!("instances_{}.json", name)),
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
            if let Some(img) = image_map.get(&image.file_name) {
                let image_path = if let Some(img_rel_path_map) = image_rel_path {
                    if let Some(rel) = img_rel_path_map.get(&image.file_name) {
                        input.join(rel).join(img)
                    } else {
                        return log_err!(format!(
                            "{} is not in the mapping to get the path",
                            &image.file_name
                        ));
                    }
                } else {
                    input.join(img)
                };
                if let Err(err) = copy(
                    image_path,
                    output.join(IMAGE_PATH).join(&name).join(&image.file_name),
                ) {
                    return log_err!(format!(
                        "Failed to copy '{}' to '{}': {}",
                        input.join(img).display(),
                        output
                            .join(IMAGE_PATH)
                            .join(&name)
                            .join(image.file_name.clone())
                            .display(),
                        err
                    ));
                }
            } else {
                return log_err!(format!("Image does not exist: {}", image.file_name));
            }
        }
    }

    Ok(())
}
