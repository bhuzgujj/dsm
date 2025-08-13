use crate::models::datasets::classes::Classes;
use crate::models::datasets::licence::License;
use crate::models::form::DataFormat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetaData {
	name: String,
	version: String,
	subset_version: Option<String>,
	contributor: String,
	date_created: String,
	description: String,
	is_incomplete: bool,
	url: String,
	year: String,
	formatter: DataFormat,
	classes: HashMap<u32, Classes>,
	licenses: HashMap<u32, License>,
	contained_in_merged: Vec<String>,
}

impl MetaData {
	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn version(&self) -> &String {
		&self.version
	}

	pub fn classes(&self) -> &HashMap<u32, Classes> {
		&self.classes
	}

	pub fn subset_version(&self) -> &Option<String> {
		&self.subset_version
	}

	pub fn contributor(&self) -> &String {
		&self.contributor
	}

	pub fn date_created(&self) -> &String {
		&self.date_created
	}

	pub fn description(&self) -> &String {
		&self.description
	}

	pub fn url(&self) -> &String {
		&self.url
	}

	pub fn year(&self) -> &String {
		&self.year
	}

	pub fn formatter(&self) -> &DataFormat {
		&self.formatter
	}

	pub fn licenses(&self) -> &HashMap<u32, License> {
		&self.licenses
	}

	pub fn is_incomplet(&self) -> &bool {
		&self.is_incomplete
	}

	pub fn contained_in_merged(&self) -> &Vec<String> {
		&self.contained_in_merged
	}
}

pub struct MetaDataBuilder {
	name: String,
	version: String,
	subset_version: Option<String>,
	contributor: String,
	date_created: String,
	description: String,
	url: String,
	year: String,
	is_incomplete: bool,
	formatter: DataFormat,
	classes: HashMap<u32, Classes>,
	licenses: HashMap<u32, License>,
	contained_in_merged: Vec<String>,
}

impl MetaDataBuilder {
	pub fn new(
		name: String,
		version: String,
		formatter: DataFormat,
		classes: HashMap<u32, Classes>,
	) -> Self {
		Self {
			name,
			version,
			subset_version: None,
			contributor: String::new(),
			date_created: String::new(),
			description: String::new(),
			url: String::new(),
			is_incomplete: false,
			year: String::new(),
			formatter,
			classes,
			licenses: HashMap::new(),
			contained_in_merged: Vec::new(),
		}
	}

	pub fn set_contributor(mut self, contributor: String) -> Self {
		self.contributor = contributor;
		self
	}

	pub fn set_date_created(mut self, date_created: String) -> Self {
		self.date_created = date_created;
		self
	}

	pub fn set_description(mut self, description: String) -> Self {
		self.description = description;
		self
	}

	pub fn set_url(mut self, url: String) -> Self {
		self.url = url;
		self
	}

	pub fn set_year(mut self, year: String) -> Self {
		self.year = year;
		self
	}

	pub fn set_licenses(mut self, licenses: HashMap<u32, License>) -> Self {
		self.licenses = licenses;
		self
	}

	pub fn set_classes(mut self, classes: HashMap<u32, Classes>) -> Self {
		self.classes = classes;
		self
	}

	pub fn set_incomplete(mut self, is_incomplete: bool) -> Self {
		self.is_incomplete = is_incomplete;
		self
	}

	pub fn set_contained_in_merged(mut self, contained_in_merged: Vec<String>) -> Self {
		self.contained_in_merged = contained_in_merged;
		self
	}

	pub fn add_contained_in_merged(mut self, merged: String) -> Self {
		if self.contained_in_merged.contains(&merged) {
			return self;
		}
		self.contained_in_merged.push(merged);
		self
	}

	pub fn build(self) -> MetaData {
		MetaData {
			name: self.name,
			version: self.version,
			subset_version: self.subset_version,
			contributor: self.contributor,
			date_created: self.date_created,
			description: self.description,
			url: self.url,
			year: self.year,
			formatter: self.formatter,
			classes: self.classes,
			is_incomplete: self.is_incomplete,
			licenses: self.licenses,
			contained_in_merged: self.contained_in_merged,
		}
	}
}

impl From<MetaData> for MetaDataBuilder {
	fn from(value: MetaData) -> Self {
		Self {
			name: value.name,
			version: value.version,
			subset_version: value.subset_version,
			contributor: value.contributor,
			date_created: value.date_created,
			description: value.description,
			url: value.url,
			year: value.year,
			formatter: value.formatter,
			classes: value.classes,
			is_incomplete: value.is_incomplete,
			licenses: value.licenses,
			contained_in_merged: value.contained_in_merged,
		}
	}
}
