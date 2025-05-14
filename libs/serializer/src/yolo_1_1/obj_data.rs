use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;
use log::warn;
use crate::yolo_1_1::{strip_prefix, PREFIX};

const FILE_NAME: &str = "obj.data";

#[derive(Debug)]
pub(crate) struct ObjData {
	classes: u32,
	names: String,
	backup: Option<String>,
	sets: HashMap<String, String>,
}

impl ObjData {
	pub(crate) fn read(path: &PathBuf) -> anyhow::Result<Self> {
		let abs_path = path.join(FILE_NAME);
		let contents = read_to_string(&abs_path)?;
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
							backup = Some(value.to_string());
						},
						"classes" => {
							if classes.is_some() {
								warn!("Duplicate classes in line '{}' of '{}'", nline, &abs_path.display());
							}
							classes = Some(u32::from_str(value)
								.expect(format!("Invalid class value, '{}' should be a integer!", value).as_str()));
						},
						"names" => {
							let value = strip_prefix(value);
							if names.is_some() {
								warn!("Duplicate names in line '{}' of '{}'", nline, &abs_path.display());
							}
							names = Some(value.to_string());
						},
						_ => {
							let value = strip_prefix(value);
							if sets.contains_key(&key.to_string()) {
								warn!("Duplicate names in line '{}' of '{}'", nline, &abs_path.display());
							}
							let _ = sets.insert(key.to_string(), value);
						}
					}
				}
			}
		}
		Ok(Self {
			classes: classes.expect(format!("Missing 'classes' in '{}'", abs_path.display()).as_str()),
			names: names.expect(format!("Missing 'names' in '{}'", abs_path.display()).as_str()),
			backup,
			sets,
		})
	}

	pub(crate) fn get_sets(&self) -> &HashMap<String, String> {
		&self.sets
	}

	pub(crate) fn get_names(&self) -> String {
		self.names.clone()
	}
}