mod dsm_sets;
mod merged_sets;

use bhomz::log_err;
use log::debug;
use serde::{de::DeserializeOwned, Serialize};
use std::{
	fs::{self, read_to_string},
	path::{Path, PathBuf},
};

use interfaces::{models::actions::Action, namable::Namable};

pub const RAW_SET: &str = "raw_sets";
pub const MERGED_SET: &str = "merged_sets";
pub const SEPARATOR: &str = "~";
pub const FORMAT_FILE_NAME: &str = "formats.dsm.txt";

pub trait Storable
where
	Self: DeserializeOwned + Serialize + Namable,
{
	/// Store a cache version of the datasets information.
	fn ledge(&self, ledger_directory: &Path) -> anyhow::Result<bool>;

	/// Store the actual data of said datasets
	fn store(&self, store_directory: &Path, originals_path: &Path) -> anyhow::Result<PathBuf>;

	/// Read the datasets information from the ledger
	fn read(ledger_directory: &Path, name: String, version: String)
		-> anyhow::Result<Option<Self>>;

	/// Get all datasets information logged in the ledger
	fn list(ledger_directory: &Path) -> anyhow::Result<Vec<Self>>;

	/// Convert dataset into an action
	fn to_action(&self) -> Action;
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

#[inline(always)]
fn read_dsm<T: DeserializeOwned + Namable>(path: &Path) -> anyhow::Result<T> {
	debug!("Parsing '{}' into a {}", path.display(), T::type_name());
	let content = match read_to_string(path) {
		Ok(ctnt) => ctnt,
		Err(err) => return log_err!(format!("Failed to read {}: {}", path.display(), err)),
	};
	match serde_json::from_str(content.as_str()) {
		Ok(datasets) => Ok(datasets),
		Err(err) => log_err!(format!("Failed to deserialize {}: {}", path.display(), err)),
	}
}
