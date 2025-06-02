mod dsm_sets;
mod merged_sets;

use std::{fs::{self, read_to_string}, path::Path};

use serde::{de::DeserializeOwned, Serialize};

use interfaces::logger::error;

use crate::storable::Storable;

pub const RAW_SET: &str = "raw_sets";
pub const MERGED_SET: &str = "merged_sets";

pub trait Localable
where
    Self: DeserializeOwned + Serialize + Storable,
{
    fn ledge(&self, ledger_directory: &Path) -> anyhow::Result<()>;
    fn store(
        &self,
        store_directory: &Path,
        originals_path: &std::path::Path,
    ) -> anyhow::Result<()>;
    fn read(
        ledger_directory: &Path,
        name: String,
        version: String,
    ) -> anyhow::Result<Option<Self>>;
    fn list(ledger_directory: &Path) -> anyhow::Result<Vec<Self>>;
}

fn copy_recursively(src: &Path, dst: &Path) -> anyhow::Result<()> {
	fs::create_dir_all(dst)?;
	for entry in fs::read_dir(src)? {
		let entry = entry?;
		let ty = entry.file_type()?;
		if ty.is_dir() {
			copy_recursively(&entry.path(), &dst.join(entry.file_name()))?;
		} else {
			fs::copy(entry.path(), dst.join(entry.file_name()))?;
		}
	}
	Ok(())
}

#[inline]
fn read_dsm<T: DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
	let content = match read_to_string(path) {
		Ok(ctnt) => ctnt,
		Err(err) => return error(format!("Failed to read {}: {}", path.display(), err))
	};
	match serde_json::from_str(content.as_str()) {
		Ok(datasets) => Ok(datasets),
		Err(err) => error(format!("Failed to deserialize {}: {}", path.display(), err))
	}
}