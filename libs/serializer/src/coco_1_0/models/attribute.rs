use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct CocoAttribute {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) occluded: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) rotation: Option<f64>,
}

impl CocoAttribute {
	pub(crate) fn new(occluded: Option<bool>, rotation: Option<f64>) -> Option<Self> {
		if occluded.is_none() && rotation.is_none() {
			return None;
		}
		Some(Self { occluded, rotation })
	}
}
