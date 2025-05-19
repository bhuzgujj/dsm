use crate::models::{ClassMapping, DataForm, Datasets};
use std::collections::HashMap;
use crate::models::classes::Classes;
use crate::models::entries::DatasetEntry;
use crate::models::licence::License;
use crate::models::metadata::MetaData;
use crate::models::storable_merged::StorableMerged;

#[derive(Clone)]
pub struct MergedSet {
	datasets: Vec<Datasets>,
	name: String,
	version: u32,
	mapping: ClassMapping,
	date_created: String,
	description: String,
	licenses: HashMap<u32, License>,
}

impl MergedSet {
	pub fn new(
		datasets: HashMap<String, Datasets>,
	    name: String,
	    version: u32,
	    mapping: ClassMapping
	) -> anyhow::Result<Self> {
		let sets: Vec<Datasets> = datasets.values().cloned().collect();
		let date_created: Option<String> = None;
		let description: Option<String> = None;
		let licenses: HashMap<u32, License> = HashMap::new();
		for dataset in &sets {
			mapping.validate(dataset)?;
		}
		Ok(Self {
			datasets: sets,
			name,
			version,
			mapping,
			date_created: date_created.unwrap_or(String::new()),
			description: description.unwrap_or(String::new()),
			licenses
		})
	}

	pub fn from_vec(
		datasets: Vec<Datasets>,
		name: String,
		version: u32,
		mapping: ClassMapping
	) -> anyhow::Result<Self> {
		let date_created: Option<String> = None;
		let description: Option<String> = None;
		let licenses: HashMap<u32, License> = HashMap::new();
		for dataset in &datasets {
			mapping.validate(dataset)?;
		}
		Ok(Self {
			datasets,
			name,
			version,
			mapping,
			date_created: date_created.unwrap_or(String::new()),
			description: description.unwrap_or(String::new()),
			licenses
		})
	}

	pub fn get_name(&self) -> &str {
		&self.name
	}

	pub fn get_version(&self) -> String {
		self.version.to_string()
	}

	pub fn to_dataset(&self, form: DataForm) -> anyhow::Result<Datasets> {
		let metadata = MetaData::new(
			self.name.clone(),
			self.version,
			None,
			String::new(),
			self.date_created.clone(),
			self.description.clone(),
			String::new(),
			String::new(),
			form,
			self.mapping.classes.iter().fold(HashMap::new(), |mut acc, (class, index)| {
				acc.insert(*index, Classes::new(class.clone(), None));
				acc
			}),
			self.licenses.clone()
		);
		let mut data_entries: HashMap<String, Vec<DatasetEntry>> = HashMap::new();
		for dataset in &self.datasets {
			for (name, entries) in dataset.get_entries() {
				if !data_entries.contains_key(name) {
					data_entries.insert(name.to_string(), Vec::new());
				}
				for entry in entries {
					let new_entry = entry.map_in(dataset.get_name(), dataset.get_version(), &self.mapping, dataset.get_classes())?;
					data_entries.get_mut(name)
						.expect("data entry exists")
						.push(new_entry);
				}
			}
		}

		Ok(Datasets::new(metadata, data_entries))
	}

	pub fn to_storable(&self) -> StorableMerged {
		let mut sets = Vec::new();

		for dataset in &self.datasets {
			sets.push(format!("{}-v{}.json", dataset.get_name(), dataset.get_version()));
		}

		StorableMerged {
			datasets: sets,
			mapping: self.mapping.clone()
		}
	}
}