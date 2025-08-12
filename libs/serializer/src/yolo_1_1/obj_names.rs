use interfaces::log_err;
use interfaces::models::datasets::Classes;
use interfaces::paths::write_to_file;
use log::debug;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub(crate) const FILE_NAME: &str = "obj.names";

pub(crate) fn read(path: PathBuf) -> anyhow::Result<HashMap<u32, Classes>> {
	debug!("Reading '{}'", path.display());
	let contents = match read_to_string(&path) {
		Ok(contents) => contents,
		Err(err) => {
			return log_err!(format!("Failed to read '{}': {err}", path.display()));
		},
	};
	let mut classes = HashMap::new();
	let mut index: u32 = 0;
	for line in contents.lines() {
		let trimmed = line.trim();
		if !trimmed.is_empty() {
			classes.insert(index, Classes::new(trimmed.to_string(), None));
			index += 1;
		}
	}
	Ok(classes)
}

pub(crate) fn write(root: &Path, classes: HashMap<u32, Classes>) -> anyhow::Result<()> {
	let mut vec: Vec<String> = Vec::with_capacity(classes.capacity());
	for i in 0..classes.len() {
		let index = i as u32;
		vec.push(classes.get(&index).unwrap().class().clone());
	}
	let path_buf = root.join(FILE_NAME);
	write_to_file(&path_buf, vec.join("\n"), true, true)
}
