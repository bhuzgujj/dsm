use crate::yolo_1_1::obj_names;
use crate::yolo_1_1::strip_prefix;
use interfaces::log_err;
use interfaces::paths::write_to_file;
use log::{debug, warn};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

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
			classes,
			names: String::from(obj_names::FILE_NAME),
			sets,
			backup: None,
		}
	}

	pub(crate) fn write(&self, root: &Path) -> anyhow::Result<()> {
		let mut content = format!("classes = {}\nnames = {}\n", self.classes, self.names);
		for (k, v) in self.sets.iter() {
			content.push_str(&format!("{k} = {v}\n"));
		}
		if let Some(backup) = &self.backup {
			content.push_str(&format!("backup = {backup}\n"));
		}
		debug!("Writting to '{}'", root.join(FILE_NAME).display());
		write_to_file(&root.join(FILE_NAME), content, true, true)
	}

	pub(crate) fn read(path: &Path) -> anyhow::Result<Self> {
		let abs_path = path.join(FILE_NAME);
		debug!("Reading '{}'", abs_path.display());
		let contents = match read_to_string(&abs_path) {
			Ok(contents) => contents,
			Err(err) => {
				return log_err!(format!(
					"error! reading file '{}': {}",
					abs_path.display(),
					err
				));
			},
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
					warn!(
						"Missing '=' in line '{}' of '{}'",
						nline,
						&abs_path.display()
					);
				} else {
					let key = parts[0].trim();
					let value = parts[1].trim();
					match key {
						"backup" => {
							if backup.is_some() {
								warn!(
									"Duplicate backup in line '{}' of '{}'",
									nline,
									&abs_path.display()
								);
							}
							debug!("'backup' has been read as '{value}'");
							backup = Some(value.to_string());
						},
						"classes" => {
							if classes.is_some() {
								warn!(
									"Duplicate classes in line '{}' of '{}'",
									nline,
									&abs_path.display()
								);
							}
							debug!("'classes' has been read as '{value}'");
							classes = Some(u32::from_str(value).unwrap_or_else(|_| {
								panic!("Invalid class value, '{value}' should be a integer!")
							}));
						},
						"names" => {
							let value = strip_prefix(value);
							if names.is_some() {
								warn!(
									"Duplicate names in line '{}' of '{}'",
									nline,
									&abs_path.display()
								);
							}
							debug!("'names' has been read as '{value}'");
							names = Some(value.to_string());
						},
						_ => {
							let value = strip_prefix(value);
							if sets.contains_key(key) {
								warn!(
									"Duplicate names in line '{}' of '{}'",
									nline,
									&abs_path.display()
								);
							}
							debug!("'{key}' has been read as '{value}'");
							let _ = sets.insert(key.to_string(), value);
						},
					}
				}
			}
		}
		if classes.is_none() {
			return log_err!(format!("Missing 'classes' in '{}'", abs_path.display()));
		}
		if names.is_none() {
			return log_err!(format!("Missing 'names' in '{}'", abs_path.display()));
		}
		Ok(Self {
			classes: classes
				.unwrap_or_else(|| panic!("Missing 'classes' in '{}'", abs_path.display())),
			names: names.unwrap_or_else(|| panic!("Missing 'names' in '{}'", abs_path.display())),
			backup,
			sets,
		})
	}

	pub(crate) fn sets(&self) -> &HashMap<String, String> {
		&self.sets
	}

	pub(crate) fn names(&self) -> &String {
		&self.names
	}
}
