use crate::models::{classes::DsmClasses, DsmSets};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, vec};
use std::path::Path;
use log::trace;
use crate::log_err;
use crate::paths::read_from_file;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMapper {
	classes: HashMap<String, u32>,
	mapping: HashMap<String, Vec<String>>,
	custom: Option<HashMap<String, CustomClassMapping>>,
}

impl ClassMapper {
	pub fn classes(&self) -> &HashMap<String, u32> {
		&self.classes
	}

	pub fn mapping(&self) -> &HashMap<String, Vec<String>> {
		&self.mapping
	}

	pub fn custom(&self) -> &Option<HashMap<String, CustomClassMapping>> {
		&self.custom
	}
}

impl ClassMapper {
	pub fn read_from_file(file_path: &Path) -> anyhow::Result<ClassMapper> {
		let content = read_from_file(file_path)?;
		match toml::from_str(&content) {
			Ok(mapping) => Ok(mapping),
			Err(err) => {
				log_err!(format!(
                    "Could not deserialize toml mapping file '{}': {err}",
                    &file_path.display()
                ))
			}
		}
	}

	pub(crate) fn get_class_for(&self, name: &String, class: &DsmClasses) -> Option<&u32> {
		if let Some(custom) = &self.custom {
			if let Some(k) = custom.get(name) {
				if let Some(new_name) =  k.mapping.get(class.class()) {
					return self.classes.get(new_name);
				} else {
					trace!("Custom mapping does not have '{}', will be ignored", class.class());
				}
			} else {
				trace!("'{}' does not have a custom mapping", name);
			}
		}
		for (key, map) in self.mapping.iter() {
			if map.contains(class.class()) {
				return self.classes.get(key);
			} else {
				trace!("Could not map '{}' with {key}", class.class());
			}
		}
		None
	}

	pub(crate) fn get_dsm_classes(&self) -> HashMap<u32, DsmClasses> {
		self.classes.iter().fold(HashMap::<u32, DsmClasses>::new(), |mut acc, (class, index)| {
			acc.insert(*index, DsmClasses::new(class.clone(), None));
			acc
		})
	}
}

impl From<&DsmSets> for ClassMapper {
	fn from(value: &DsmSets) -> Self {
		let mut classes = HashMap::new();
		let mut mapping = HashMap::new();
		for (index, class) in value.classes() {
			let name = class.subclass().clone()
				.unwrap_or(class.class().clone());
			classes.insert(name.clone(), *index);
			mapping.insert(name.clone(), vec![name]);
		}
		Self { 
			classes, 
			mapping, 
			custom: None
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomClassMapping {
	mapping: HashMap<String, String>,
}

impl CustomClassMapping {
	pub fn mapping(&self) -> &HashMap<String, String> {
		&self.mapping
	}
}
