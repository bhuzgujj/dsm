use crate::models::datasets::{Dataset, License};
use log::trace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseMapper {
	current_id: u32,
	licences: HashMap<u32, License>,
	mapping: HashMap<String, HashMap<u32, u32>>,
}

impl LicenseMapper {
	pub(crate) fn new() -> Self {
		let mut licences = HashMap::new();
		licences.insert(0, License::new(String::new(), String::new()));
		Self {
			current_id: 1,
			licences,
			mapping: HashMap::new(),
		}
	}

	pub(crate) fn add_licence(&mut self, id: u32, set_name: String, license: License) {
		let mut remap = true;
		let mut new_id = self.current_id;
		for licenses in self.licences.values() {
			if &license == licenses {
				new_id -= 1;
				remap = false;
				break;
			}
		}
		if let Some(mapping) = self.mapping.get_mut(&set_name) {
			mapping.insert(id, new_id);
		} else {
			trace!("Adding {set_name} in the licence mapping");
			let mut m = HashMap::new();
			m.insert(id, new_id);
			self.mapping.insert(set_name, m);
		}
		if remap {
			self.licences.insert(new_id, license.clone());
			self.current_id += 1;
		}
	}

	pub(crate) fn map_licence(&self, id: u32, set_name: String) -> Option<&u32> {
		self.mapping.get(&set_name).and_then(|map| map.get(&id))
	}

	pub(crate) fn map_licence_borrow(&self, id: u32, set_name: &String) -> Option<&u32> {
		self.mapping.get(set_name).and_then(|map| map.get(&id))
	}

	pub fn licences(&self) -> &HashMap<u32, License> {
		&self.licences
	}

	pub fn mapping(&self) -> &HashMap<String, HashMap<u32, u32>> {
		&self.mapping
	}
}

impl From<&Dataset> for LicenseMapper {
	fn from(dataset: &Dataset) -> Self {
		let licences = dataset.licenses().clone();
		let mut mapper = Self {
			current_id: licences.len() as u32,
			licences,
			mapping: HashMap::new(),
		};
		for (id, licence) in dataset.licenses() {
			mapper.add_licence(*id, dataset.name().clone(), licence.clone());
		}
		mapper
	}
}

impl From<&mut Dataset> for LicenseMapper {
	fn from(dataset: &mut Dataset) -> Self {
		let licences = dataset.licenses().clone();
		let mut mapper = Self {
			current_id: licences.len() as u32,
			licences,
			mapping: HashMap::new(),
		};
		for (id, licence) in dataset.licenses() {
			mapper.add_licence(*id, dataset.name().clone(), licence.clone());
		}
		mapper
	}
}
