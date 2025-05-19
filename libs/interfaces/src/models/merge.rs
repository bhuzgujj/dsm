use crate::models::classes::DsmClasses;
use crate::models::entries::DsmEntry;
use crate::models::metadata::DsmMetaData;
use crate::models::storable_merged::StorableMerged;
use crate::models::{ClassMapper, DataForm, DsmSets, LicenseMapper};
use std::collections::HashMap;

#[derive(Clone)]
pub struct MergedSet {
	datasets: Vec<DsmSets>,
	name: String,
	version: u32,
	class_mapper: ClassMapper,
	license_mapper: LicenseMapper,
	date_created: String,
	description: String,
}

impl MergedSet {
	pub fn new(
		datasets: HashMap<String, DsmSets>,
		name: String,
		version: u32,
		class_mapper: ClassMapper
	) -> anyhow::Result<Self> {
		let sets: Vec<DsmSets> = datasets.values().cloned().collect();
		let date_created: Option<String> = None;
		let description: Option<String> = None;
		let mut license_mapper = LicenseMapper::new();
		for dataset in &sets {
			class_mapper.validate(dataset)?;
			for (id, licence) in dataset.get_license() {
				license_mapper.add_licence(*id, dataset.get_name(), licence.clone());
			}
		}
		Ok(Self {
			datasets: sets,
			name,
			version,
			class_mapper,
			date_created: date_created.unwrap_or(String::new()),
			description: description.unwrap_or(String::new()),
			license_mapper
		})
	}

	pub fn from_vec(
		datasets: Vec<DsmSets>,
		name: String,
		version: u32,
		class_mapper: ClassMapper,
		license_mapper: LicenseMapper
	) -> anyhow::Result<Self> {
		let date_created: Option<String> = None;
		let description: Option<String> = None;
		for dataset in &datasets {
			class_mapper.validate(dataset)?;
		}
		Ok(Self {
			datasets,
			name,
			version,
			class_mapper,
			date_created: date_created.unwrap_or(String::new()),
			description: description.unwrap_or(String::new()),
			license_mapper
		})
	}

	pub fn get_name(&self) -> &str {
		&self.name
	}

	pub fn get_version(&self) -> String {
		self.version.to_string()
	}

	pub fn to_dataset(&self, form: DataForm) -> anyhow::Result<DsmSets> {
		let metadata = DsmMetaData::new(
			self.name.clone(),
			self.version,
			None,
			String::new(),
			self.date_created.clone(),
			self.description.clone(),
			String::new(),
			String::new(),
			form,
			self.class_mapper.classes.iter().fold(HashMap::new(), |mut acc, (class, index)| {
				acc.insert(*index, DsmClasses::new(class.clone(), None));
				acc
			}),
			self.license_mapper.get_licences().clone()
		);
		let mut data_entries: HashMap<String, Vec<DsmEntry>> = HashMap::new();
		for dataset in &self.datasets {
			for (name, entries) in dataset.get_entries() {
				if !data_entries.contains_key(name) {
					data_entries.insert(name.to_string(), Vec::new());
				}
				for entry in entries {
					let new_entry = entry.map_in(
						dataset.get_name(),
						dataset.get_version(),
						&self.class_mapper,
						&self.license_mapper,
						dataset.get_classes()
					)?;
					data_entries.get_mut(name)
						.expect("data entry exists")
						.push(new_entry);
				}
			}
		}

		Ok(DsmSets::new(metadata, data_entries))
	}

	pub fn to_storable(&self) -> StorableMerged {
		let mut sets = Vec::new();

		for dataset in &self.datasets {
			sets.push(format!("{}-v{}.json", dataset.get_name(), dataset.get_version()));
		}

		StorableMerged {
			datasets: sets,
			class_mapper: self.class_mapper.clone(),
			license_mapper: self.license_mapper.clone(),
		}
	}
}