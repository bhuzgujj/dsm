use std::path::PathBuf;
use crate::models::datasets::annotation::Annotation;

#[derive(Debug, Clone)]
pub struct DatasetEntry {
    image_path: PathBuf,
    width: u32,
    height: u32,
    annotation: Vec<Annotation>,
}

impl DatasetEntry {
    pub fn new(
        image_path: PathBuf,
        width: u32,
        height: u32,
        annotation: Vec<Annotation>
    ) -> Self {
        Self {
            image_path,
            width,
            height,
            annotation
        }
    }

    pub fn set_image_path(&mut self, image_path: PathBuf) {
        self.image_path = image_path;
    }

    pub fn get_image_path(&self) -> PathBuf {
        self.image_path.clone()
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
}