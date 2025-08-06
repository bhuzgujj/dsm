use crate::models::{annotation::PyAnnotation, DsmToPy};
use interfaces::models::entries::DsmEntry;
use pyo3::pyclass;
use pyo3_stub_gen_derive::gen_stub_pyclass;
use std::path::PathBuf;

#[gen_stub_pyclass]
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
			file_name: self.file_name().clone(),
			image_absolute_path: self.image_location(),
			width: *self.width(),
			height: *self.height(),
			license: *self.license(),
			flickr_url: self.flickr_url().clone(),
			coco_url: self.coco_url().clone(),
			date_captured: self.date_captured().clone(),
			annotation: self.annotation().to_py(),
		}
	}
}
