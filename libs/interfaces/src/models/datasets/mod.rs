use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::Location;

use super::{ClassMapper, LicenseMapper};

mod annotation;
mod classes;
mod entries;
mod licence;
mod metadata;

pub use annotation::*;
pub use classes::*;
pub use entries::*;
pub use licence::*;
pub use metadata::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dataset {
	pub metadata: MetaData,
	pub entries: HashMap<String, Vec<Entry>>,
}

impl Dataset {
	pub fn new(metadata: MetaData, entries: HashMap<String, Vec<Entry>>) -> Dataset {
		Self { metadata, entries }
	}

	pub fn name(&self) -> &String {
		self.metadata.name()
	}

	pub fn keyed_name(&self) -> String {
		format!("{}={}", self.metadata.name(), self.metadata.version())
	}

	pub fn metadata(&self) -> &MetaData {
		&self.metadata
	}

	pub fn licenses(&self) -> &HashMap<u32, License> {
		self.metadata.licenses()
	}

	pub fn version(&self) -> &String {
		self.metadata.version()
	}

	pub fn entries_mut(&mut self) -> &mut HashMap<String, Vec<Entry>> {
		&mut self.entries
	}

	pub fn entries(&self) -> &HashMap<String, Vec<Entry>> {
		&self.entries
	}

	pub fn classes(&self) -> &HashMap<u32, Classes> {
		self.metadata.classes()
	}

	pub fn is_incomplet(&self) -> &bool {
		self.metadata.is_incomplet()
	}

	pub fn class_count(&self) -> HashMap<u32, u32> {
		let mut class_count: HashMap<u32, u32> = HashMap::new();
		for (_, entry) in self.entries.iter() {
			for entry in entry {
				for annotation in entry.annotation() {
					if class_count.contains_key(annotation.class()) {
						class_count.insert(
							*annotation.class(),
							class_count.get(annotation.class()).unwrap() + 1,
						);
					} else {
						class_count.insert(*annotation.class(), 1);
					}
				}
			}
		}
		class_count
	}

	pub fn entry_count(&self) -> u32 {
		let mut class_count: u32 = 0;
		for (_, entry) in self.entries.iter() {
			for _ in entry {
				class_count += 1
			}
		}
		class_count
	}

	pub fn remap(&self, map: &ClassMapper) -> anyhow::Result<Self> {
		let final_map = self.classes();
		let licence_mapper = LicenseMapper::from(self);
		let dataset_name = self.name();
		let mut new_entries = HashMap::new();
		for (name, entries) in self.entries() {
			let mut subset = Vec::new();
			for entry in entries {
				subset.push(entry.remap(dataset_name, map, &licence_mapper, final_map)?);
			}
			new_entries.insert(name.clone(), subset);
		}
		Ok(Self {
			metadata: MetaDataBuilder::from(self.metadata.clone())
				.set_classes(map.get_dsm_classes())
				.build(),
			entries: new_entries,
		})
	}

	pub fn update_location(&mut self, save_location: Location) {
		for (_, entries) in self.entries.iter_mut() {
			for entry in entries.iter_mut() {
				entry.update_location(&save_location);
			}
		}
	}
}
