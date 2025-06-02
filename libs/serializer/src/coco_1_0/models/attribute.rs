use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Attribute {
	pub(crate) occluded: Option<bool>,
	pub(crate) rotation: Option<f64>
}