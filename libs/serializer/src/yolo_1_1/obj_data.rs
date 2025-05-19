use std::collections::HashMap;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use log::{debug, error, warn};
use crate::yolo_1_1::{strip_prefix};
use crate::yolo_1_1::obj_names;

const FILE_NAME: &str = "obj.data";

#[derive(Debug)]
pub(crate) struct ObjData {
	classes: u32,
	names: String,
	backup: Option<String>,
	sets: HashMap<String, String>,
}

impl ObjData {
	pub(crate) fn new(classes: u32, sets: HashMap<String, String>) -> Self {
		Self {
			classes, names: String::from(obj_names::FILE_NAME), sets, backup: None
		}
	}

	pub(crate) fn write(&self, root: &PathBuf) -> anyhow::Result<()> {
		let mut content = format!("classes = {}\nnames = {}\n", self.classes, self.names);
		for (k, v) in self.sets.iter() {
			content.push_str(&format!("{} = {}\n", k, v));
		}
		if let Some(backup) = &self.backup {
			content.push_str(&format!("backup = {}\n", backup));
		}
		debug!("Writting to '{}'", root.join(FILE_NAME).display());
		match OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(root.join(FILE_NAME)) {
			Ok(mut file) => {
				file.write_all(content.as_bytes())?;
			}
			Err(err) => {
				error!("Error writting to file '{}': {}", root.join(FILE_NAME).display(), err);
				return Err(err.into());
			}
		}
		Ok(())
	}

	pub(crate) fn read(path: &PathBuf) -> anyhow::Result<Self> {
		let abs_path = path.join(FILE_NAME);
		debug!("Reading '{}'", abs_path.display());
		let contents = match read_to_string(&abs_path) {
			Ok(contents) => contents,
			Err(err) => {
				error!("Error reading file '{}': {}", abs_path.display(), err);
				return Err(err.into());
			}
		};
		let mut classes: Option<u32> = None;
		let mut names: Option<String> = None;
		let mut backup: Option<String> = None;
		let mut sets: HashMap<String, String> = HashMap::new();
		for line in contents.lines() {
			let nline = line.trim();
			if !nline.is_empty() {
				let parts: Vec<&str> = nline.split('=').collect();
				if parts.len() != 2 {
					warn!("Missing '=' in line '{}' of '{}'", nline, &abs_path.display());
				} else {
					let key = parts[0].trim();
					let value = parts[1].trim();
					match key {
						"backup" => {
							if backup.is_some() {
								warn!("Duplicate backup in line '{}' of '{}'", nline, &abs_path.display());
							}
							debug!("'backup' has been read as '{}'", value);
							backup = Some(value.to_string());
						},
						"classes" => {
							if classes.is_some() {
								warn!("Duplicate classes in line '{}' of '{}'", nline, &abs_path.display());
							}
							debug!("'classes' has been read as '{}'", value);
							classes = Some(u32::from_str(value)
								.unwrap_or_else(|_| panic!("Invalid class value, '{}' should be a integer!", value)));
						},
						"names" => {
							let value = strip_prefix(value);
							if names.is_some() {
								warn!("Duplicate names in line '{}' of '{}'", nline, &abs_path.display());
							}
							debug!("'names' has been read as '{}'", value);
							names = Some(value.to_string());
						},
						_ => {
							let value = strip_prefix(value);
							if sets.contains_key(&key.to_string()) {
								warn!("Duplicate names in line '{}' of '{}'", nline, &abs_path.display());
							}
							debug!("'{}' has been read as '{}'", key, value);
							let _ = sets.insert(key.to_string(), value);
						}
					}
				}
			}
		}
		Ok(Self {
			classes: classes.unwrap_or_else(|| panic!("Missing 'classes' in '{}'", abs_path.display())),
			names: names.unwrap_or_else(|| panic!("Missing 'names' in '{}'", abs_path.display())),
			backup,
			sets,
		})
	}

	pub(crate) fn get_sets(&self) -> &HashMap<String, String> {
		&self.sets
	}

	pub(crate) fn get_names(&self) -> &String {
		&self.names
	}
}