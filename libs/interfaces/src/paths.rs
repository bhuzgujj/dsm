use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::log_err;

const DSM_DIR: &str = ".dsm";

#[inline]
pub fn dsm_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(DSM_DIR)
}

#[inline]
pub fn write_to_file(file_path: &Path, content: String, truncate: bool, create: bool) -> anyhow::Result<()> {
    match OpenOptions::new()
        .write(true)
        .create(create)
        .truncate(truncate)
        .open(file_path) {
        Ok(mut file) => {
            match file.write_all(content.as_bytes()) {
                Ok(_) => Ok(()),
                Err(err) => log_err!(format!("Failed to write '{}': {err}", file_path.display()))
            }
        }
        Err(err) => log_err!(format!("Failed to open file '{}': {err}", file_path.display()))
    }
}