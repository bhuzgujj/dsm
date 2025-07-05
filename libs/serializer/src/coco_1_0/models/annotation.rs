use crate::coco_1_0::models::attribute::Attribute;
use interfaces::models::annotation::{DsmAnnotation, DsmAnnotationBuilder};
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
    pub(crate) attributes: Option<Attribute>,
}

impl Annotation {
    pub(crate) fn dsm(&self) -> DsmAnnotation {
        DsmAnnotationBuilder::builder(
            self.category_id,
            self.bbox[0] + (self.bbox[2] / 2f64),
            self.bbox[1] + (self.bbox[3] / 2f64),
            self.bbox[2],
            self.bbox[3],
        )
        .set_segmentation(self.segmentation.clone())
        .set_iscrowd(self.iscrowd)
        .set_occluded(self.attributes.clone()
			.map(|a| a.occluded)
			.unwrap_or(None))
        .set_rotation(self.attributes.clone()
			.map(|a| a.rotation)
			.unwrap_or(None))
        .build()
    }

    pub(crate) fn from_dsm(id: u32, image_id: u32, data_entry: &DsmAnnotation) -> Annotation {
        Self {
            id,
            image_id,
            category_id: data_entry.get_class() + 1,
            segmentation: data_entry.get_segmentation().clone(),
            area: data_entry.get_area(),
            bbox: vec![
                data_entry.get_x() - (data_entry.get_width() / 2f64),
                data_entry.get_y() - (data_entry.get_height() / 2f64),
                *data_entry.get_width(),
                *data_entry.get_height(),
            ],
            iscrowd: *data_entry.get_iscrowd(),
            attributes: Some(Attribute {
                occluded: *data_entry.get_occluded(),
                rotation: *data_entry.get_rotation(),
            }),
        }
    }
}
