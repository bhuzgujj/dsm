use crate::models::classes::DsmClasses;
use crate::models::DsmSets;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMapper {
	pub classes: HashMap<String, u32>,
	pub mapping: HashMap<String, Vec<String>>,
	pub custom: Option<HashMap<String, Custom>>,
}

impl ClassMapper {
	pub fn validate(&self, dataset: &DsmSets) -> anyhow::Result<()> {
		for (_, classes) in dataset.get_classes() {
			if let Some(custom) = &self.custom {
				if custom.get(&dataset.get_name()).is_some_and(|k| k.mapping.contains_key(classes.get_classes())) {
					continue;
				}
			}
			let mut err = true;
			for (_, map) in self.mapping.iter() {
				if map.contains(classes.get_classes()) {
					err = false;
					break;
				}
			}
			if err {
				return Err(anyhow!("Missing classes mapping: {}", classes.get_classes()));
			}
		}
		Ok(())
	}

	pub(crate) fn get_class_for(&self, name: &String, class: &DsmClasses) -> Option<&u32> {
		if let Some(custom) = &self.custom {
			if let Some(k) = custom.get(name) {
				if let Some(new_name) =  k.mapping.get(class.get_classes()) {
					return self.classes.get(new_name);
				}
			}
		}
		for (key, map) in self.mapping.iter() {
			if map.contains(class.get_classes()) {
				return self.classes.get(key);
			}
		}
		None
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Custom {
	pub mapping: HashMap<String, String>,
}