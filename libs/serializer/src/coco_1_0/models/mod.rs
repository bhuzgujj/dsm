use crate::coco_1_0::models::sequence::Sequence;
use interfaces::log_err;
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

pub mod annotation;
pub mod attribute;
pub mod category;
pub mod image;
pub mod info;
pub mod license;
pub mod sequence;

pub(crate) fn read(root: &PathBuf) -> anyhow::Result<HashMap<String, Sequence>> {
    match read_dir(root) {
        Ok(dir) => {
            let mut sequences: HashMap<String, Sequence> = HashMap::new();
            for files in dir {
                let f = files?;
                if let Ok(content) = read_to_string(f.path()) {
                    let sequence: Sequence = match serde_json::from_str(content.as_str()) {
                        Ok(s) => s,
                        Err(err) => {
                            return log_err!(format!(
                                "Failed to deserialize Sequence '{}': {}",
                                f.path().display(),
                                err
                            ))
                        }
                    };
                    sequences.insert(f.file_name().to_string_lossy().to_string(), sequence);
                }
            }
            Ok(sequences)
        }
        Err(err) => {
            log_err!(format!(
                "Failed to read dataset directory '{}': {err}",
                root.display()
            ))
        }
    }
}
