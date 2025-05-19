use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Attribute {
	pub(crate) occluded: bool,
	pub(crate) rotation: u32
}