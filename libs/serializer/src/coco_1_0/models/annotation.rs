use serde::{Deserialize, Serialize};
use crate::coco_1_0::models::attribute::Attribute;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Annotation {
	pub(crate) id: u32,
	pub(crate) image_id: u32,
	pub(crate) category_id: u32,
	pub(crate) segmentation: Vec<f64>,
	pub(crate) area: f64,
	pub(crate) bbox: Vec<f64>,
	pub(crate) iscrowd: u32,
	pub(crate) attributes: Attribute
}

impl Annotation {
	pub(crate) fn to_datasettable(&self) -> interfaces::models::annotation::Annotation {
		interfaces::models::annotation::Annotation::new(
			self.category_id,
			self.bbox[0] + (self.bbox[2] / 2f64),
			self.bbox[1] + (self.bbox[3] / 2f64),
			self.bbox[2],
			self.bbox[3],
			self.segmentation.clone(),
			self.iscrowd,
			self.attributes.occluded,
			self.attributes.rotation
		)
	}
}