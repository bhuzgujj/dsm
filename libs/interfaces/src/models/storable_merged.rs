use std::collections::HashMap;

use crate::models::{ClassMapper, DsmLocation, LicenseMapper};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StorableMerged {
	pub name: String,
	pub version: String,
	pub datasets: HashMap<String, DsmLocation>,
	pub class_mapper: ClassMapper,
	pub license_mapper: LicenseMapper
}