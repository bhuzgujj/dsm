use interfaces::models::entries::DsmEntry;
use pyo3::pyclass;
use std::path::PathBuf;

use crate::models::{annotation::PyAnnotation, DsmToPy};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyEntry {
    #[pyo3(get, set)]
    file_name: String,
    #[pyo3(get, set)]
    image_absolute_path: PathBuf,
    #[pyo3(get, set)]
    width: u32,
    #[pyo3(get, set)]
    height: u32,
    #[pyo3(get, set)]
    license: Option<u32>,
    #[pyo3(get, set)]
    flickr_url: Option<String>,
    #[pyo3(get, set)]
    coco_url: Option<String>,
    #[pyo3(get, set)]
    date_captured: Option<String>,
    #[pyo3(get, set)]
    annotation: Vec<PyAnnotation>,
}

impl DsmToPy<PyEntry> for DsmEntry {
    fn to_py(&self) -> PyEntry {
        PyEntry {
            file_name: self.get_file_name().clone(),
            image_absolute_path: self.get_image_location(),
            width: *self.get_width(),
            height: *self.get_height(),
            license: self.get_license().clone(),
            flickr_url: self.get_flickr_url().clone(),
            coco_url: self.get_coco_url().clone(),
            date_captured: self.get_date_captured().clone(),
            annotation: self.get_annotation().to_py(),
        }
    }
}
