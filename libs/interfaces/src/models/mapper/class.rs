use crate::models::classes::DsmClasses;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::trace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMapper {
	pub classes: HashMap<String, u32>,
	pub mapping: HashMap<String, Vec<String>>,
	pub custom: Option<HashMap<String, Custom>>,
}

impl ClassMapper {
	pub(crate) fn get_class_for(&self, name: &String, class: &DsmClasses) -> Option<&u32> {
		if let Some(custom) = &self.custom {
			if let Some(k) = custom.get(name) {
				if let Some(new_name) =  k.mapping.get(class.get_classes_name()) {
					return self.classes.get(new_name);
				} else {
					trace!("Custom mapping does not have '{}', will be ignored", class.get_classes_name());
				}
			} else {
				trace!("'{}' does not have a custom mapping", name);
			}
		}
		for (key, map) in self.mapping.iter() {
			if map.contains(class.get_classes_name()) {
				return self.classes.get(key);
			} else {
				trace!("Could not map '{}' with {key}", class.get_classes_name());
			}
		}
		None
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Custom {
	pub mapping: HashMap<String, String>,
}