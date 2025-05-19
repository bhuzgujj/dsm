use crate::coco_1_0::models::attribute::Attribute;
use serde::{Deserialize, Serialize};

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
	pub(crate) fn to_datasettable(&self) -> interfaces::models::annotation::DsmAnnotation {
		interfaces::models::annotation::DsmAnnotation::new(
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

	pub(crate) fn from_interface(id: u32, image_id: u32, data_entry: &interfaces::models::annotation::DsmAnnotation) -> Annotation {
		Self {
			id,
			image_id,
			category_id: data_entry.get_class() + 1,
			segmentation: data_entry.get_segmentation().clone(),
			area: data_entry.get_area(),
			bbox: vec![
				data_entry.get_x() - (data_entry.get_width() / 2f64),
				data_entry.get_y() - (data_entry.get_height() / 2f64),
				data_entry.get_width(),
				data_entry.get_height(),

			],
			iscrowd: data_entry.get_iscrowd(),
			attributes: Attribute {
				occluded: data_entry.get_occluded(),
				rotation: data_entry.get_rotation(),
			}
		}
	}
}