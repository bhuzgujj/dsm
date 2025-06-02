use std::collections::HashMap;
use log::trace;
use serde::{Deserialize, Serialize};
use crate::models::{licence::DsmLicense, DsmSets};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseMapper {
	current_id: u32,
	licences: HashMap<u32, DsmLicense>,
	mapping: HashMap<String, HashMap<u32, u32>>,
}

impl LicenseMapper {
	pub(crate) fn new() -> Self {
		let mut licences = HashMap::new();
		licences.insert(0, DsmLicense::new(String::new(), String::new()));
		Self {
			current_id: 1,
			licences,
			mapping: HashMap::new(),
		}
	}

	pub(crate) fn add_licence(&mut self, id: u32, set_name: String, license: DsmLicense) {
		self.licences.insert(self.current_id, license);
		if let Some(mapping) = self.mapping.get_mut(&set_name) {
			mapping.insert(id, self.current_id);
		} else {
			trace!("Adding {set_name} in the licence mapping");
			let mut m = HashMap::new();
			m.insert(id, self.current_id);
			self.mapping.insert(set_name, m);
		}
		self.current_id += 1;
	}

	pub(crate) fn map_licence(&self, id: u32, set_name: String) -> Option<&u32> {
		self.mapping.get(&set_name).and_then(|map| map.get(&id))
	}

	pub(crate) fn map_licence_borrow(&self, id: u32, set_name: &String) -> Option<&u32> {
		self.mapping.get(set_name).and_then(|map| map.get(&id))
	}

	pub(crate) fn get_licences(&self) -> &HashMap<u32, DsmLicense> {
		&self.licences
	}
}

impl From<&DsmSets> for LicenseMapper {
	fn from(dataset: &DsmSets) -> Self {
		let licences = dataset.get_license().clone();
		let mut mapper = Self { 
			current_id: licences.len() as u32,
			licences,
			mapping: HashMap::new(),
		};
		for (id, licence) in dataset.get_license() {
			mapper.add_licence(id.clone(), dataset.get_name().clone(), licence.clone());
		}
		mapper
	}
}
impl From<&mut DsmSets> for LicenseMapper {
	fn from(dataset: &mut DsmSets) -> Self {
		let licences = dataset.get_license().clone();
		let mut mapper = Self { 
			current_id: licences.len() as u32,
			licences,
			mapping: HashMap::new(),
		};
		for (id, licence) in dataset.get_license() {
			mapper.add_licence(id.clone(), dataset.get_name().clone(), licence.clone());
		}
		mapper
	}
}