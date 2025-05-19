use crate::models::{ClassMapper, LicenseMapper};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StorableMerged {
	pub datasets: Vec<String>,
	pub class_mapper: ClassMapper,
	pub license_mapper: LicenseMapper
}