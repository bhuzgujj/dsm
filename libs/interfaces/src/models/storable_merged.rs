use crate::models::ClassMapping;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StorableMerged {
	pub datasets: Vec<String>,
	pub mapping: ClassMapping
}