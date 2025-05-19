use std::collections::HashMap;
use crate::models::datasets::annotation::Annotation;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::models::classes::Classes;
use crate::models::ClassMapping;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasetEntry {
    file_name: String,
    image_relative_path: PathBuf,
    width: u32,
    height: u32,
    license: Option<u32>,
    flickr_url: Option<String>,
    coco_url: Option<String>,
    date_captured: Option<u32>,
    annotation: Vec<Annotation>,
}

impl DatasetEntry {
    pub fn new(
        image_path: PathBuf,
        width: u32,
        height: u32,
        file_name: String,
        license: Option<u32>,
        flickr_url: Option<String>,
        coco_url: Option<String>,
        date_captured: Option<u32>,
        annotation: Vec<Annotation>
    ) -> Self {
        Self {
            image_relative_path: image_path,
            width,
            height,
            file_name,
            license,
            flickr_url,
            coco_url,
            date_captured,
            annotation
        }
    }

    pub fn get_image_relative_path(&self) -> PathBuf {
        self.image_relative_path.clone()
    }

    pub fn get_file_name(&self) -> String {
        self.file_name.clone()
    }
    
    pub fn get_width(&self) -> u32 {
        self.width
    }
    
    pub fn get_height(&self) -> u32 {
        self.height
    }
    
    pub fn get_annotation(&self) -> &Vec<Annotation> {
        &self.annotation
    }

    pub fn map_in(&self, name: String, version: u32, mapping: &ClassMapping, classes: HashMap<u32, Classes>) -> anyhow::Result<Self> {
        let prefix = format!("{}-v{}", name, version);
        let mut new_annotation = Vec::new();
        for ann in &self.annotation {
            let old_class = classes.get(&ann.get_class()).expect("TODO: Class not found");
            let new_class = mapping.get_class_for(&name, old_class).expect("TODO: Class not found");
            new_annotation.push(ann.map_in(*new_class));
        }
        Ok(Self {
            image_relative_path: self.image_relative_path.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            file_name: format!("{}-{}", prefix, self.file_name.clone()),
            license: self.license.clone(),
            flickr_url: self.flickr_url.clone(),
            coco_url: self.coco_url.clone(),
            date_captured: self.date_captured.clone(),
            annotation: new_annotation,
        })
    }
}