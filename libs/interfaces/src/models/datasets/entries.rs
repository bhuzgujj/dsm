use crate::logger::error;
use crate::models::classes::DsmClasses;
use crate::models::datasets::annotation::DsmAnnotation;
use crate::models::{ClassMapper, LicenseMapper};
use log::warn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DsmEntry {
    file_name: String,
    image_relative_path: PathBuf,
    width: u32,
    height: u32,
    license: Option<u32>,
    flickr_url: Option<String>,
    coco_url: Option<String>,
    date_captured: Option<String>,
    annotation: Vec<DsmAnnotation>,
}

impl DsmEntry {
    pub fn get_image_relative_path(&self) -> &PathBuf {
        &self.image_relative_path
    }

    pub fn get_file_name(&self) -> &String {
        &self.file_name
    }

    pub fn get_width(&self) -> &u32 {
        &self.width
    }

    pub fn get_height(&self) -> &u32 {
        &self.height
    }

    pub fn get_license(&self) -> &Option<u32> {
        &self.license
    }

    pub fn get_flickr_url(&self) -> &Option<String> {
        &self.flickr_url
    }

    pub fn get_coco_url(&self) -> &Option<String> {
        &self.coco_url
    }

    pub fn get_annotation(&self) -> &Vec<DsmAnnotation> {
        &self.annotation
    }

    pub fn get_date_captured(&self) -> &Option<String> {
        &self.date_captured
    }

    pub fn remap(
        &self,
        name: &String,
        class_mapper: &ClassMapper,
        licence_mapper: &LicenseMapper,
        classes: &HashMap<u32, DsmClasses>
    ) -> anyhow::Result<Self> {
        let mut new_annotation = Vec::new();
        for ann in &self.annotation {
            if let Some(old_class) = classes.get(ann.get_class()) {
                if let Some(new_class) = class_mapper.get_class_for(&name, old_class) {
                    new_annotation.push(ann.map_in(*new_class));
                } else {
                    warn!("No mapping for annotation ({}: {})", old_class.get_classes_name(), ann.get_class())
                }
            } else {
                return error(format!("Annotation {} no found in original mapping", ann.get_class()))
            }
        }
        let mut licence_new_id = None;
        if let Some(id) = self.license {
            licence_new_id = licence_mapper.map_licence_borrow(id, name);
        }
        Ok(Self {
            image_relative_path: self.image_relative_path.clone(),
            width: self.width,
            height: self.height,
            file_name: self.file_name.clone(),
            license: licence_new_id.copied(),
            flickr_url: self.flickr_url.clone(),
            coco_url: self.coco_url.clone(),
            date_captured: self.date_captured.clone(),
            annotation: new_annotation,
        })
    }

    pub fn map_in(
        &self,
        name: String,
        version: String,
        class_mapper: &ClassMapper,
        licence_mapper: &LicenseMapper,
        classes: HashMap<u32, DsmClasses>
    ) -> Self {
        let prefix = format!("{}-v{}", name, version);
        let mut new_annotation = Vec::new();
        for ann in &self.annotation {
            if let Some(old_class) = classes.get(ann.get_class()) {
                if let Some(new_class) = class_mapper.get_class_for(&name, old_class) {
                    new_annotation.push(ann.map_in(*new_class));
                }
            }
        }
        let mut licence_new_id = None;
        if let Some(id) = self.license {
            licence_new_id = licence_mapper.map_licence(id, name);
        }
        Self {
            image_relative_path: self.image_relative_path.clone(),
            width: self.width,
            height: self.height,
            file_name: 
                format!("{}-{}", prefix, self.file_name.clone()),
            license: licence_new_id.copied(),
            flickr_url: self.flickr_url.clone(),
            coco_url: self.coco_url.clone(),
            date_captured: self.date_captured.clone(),
            annotation: new_annotation,
        }
    }
}

pub struct DsmEntryBuilder {
    file_name: String,
    image_relative_path: PathBuf,
    width: u32,
    height: u32,
    license: Option<u32>,
    flickr_url: Option<String>,
    coco_url: Option<String>,
    date_captured: Option<String>,
    annotation: Vec<DsmAnnotation>,
}

impl DsmEntryBuilder {
    pub fn new(file_name: String, image_relative_path: PathBuf, width: u32, height: u32) -> Self {
        Self {
            file_name,
            image_relative_path,
            width,
            height,
            license: None,
            flickr_url: None,
            coco_url: None,
            date_captured: None,
            annotation: Vec::new(),
        }
    }

    pub fn set_license(mut self, license: Option<u32>) -> Self {
        self.license = license;
        self
    }

    pub fn set_flickr_url(mut self, flickr_url: Option<String>) -> Self {
        self.flickr_url = flickr_url;
        self
    }

    pub fn set_coco_url(mut self, coco_url: Option<String>) -> Self {
        self.coco_url = coco_url;
        self
    }

    pub fn set_date_captured(mut self, date_captured: Option<String>) -> Self {
        self.date_captured = date_captured;
        self
    }

    pub fn set_annotation(mut self, annotation: Vec<DsmAnnotation>) -> Self {
        self.annotation = annotation;
        self
    }

    pub fn build(self) -> DsmEntry {
        DsmEntry {
            file_name: self.file_name,
            image_relative_path: self.image_relative_path,
            width: self.width,
            height: self.height,
            license: self.license,
            flickr_url: self.flickr_url,
            coco_url: self.coco_url,
            date_captured: self.date_captured,
            annotation: self.annotation,
        }
    }
}
