use std::collections::HashMap;

use crate::models::{classes::PyClasses, licence::PyLicense, DsmToPy};
use interfaces::models::metadata::DsmMetaData;
use pyo3::pyclass;
use pyo3_stub_gen_derive::gen_stub_pyclass;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyMetaData {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    version: String,
    #[pyo3(get, set)]
    subset_version: Option<String>,
    #[pyo3(get, set)]
    contributor: String,
    #[pyo3(get, set)]
    date_created: String,
    #[pyo3(get, set)]
    description: String,
    #[pyo3(get, set)]
    is_incomplete: bool,
    #[pyo3(get, set)]
    url: String,
    #[pyo3(get, set)]
    year: String,
    #[pyo3(get, set)]
    formatter: String,
    #[pyo3(get, set)]
    classes: HashMap<u32, PyClasses>,
    #[pyo3(get, set)]
    licenses: HashMap<u32, PyLicense>,
}

impl DsmToPy<PyMetaData> for DsmMetaData {
    fn to_py(&self) -> PyMetaData {
        PyMetaData {
            name: self.name().clone(),
            version: self.version().clone(),
            subset_version: self.subset_version().clone(),
            contributor: self.contributor().clone(),
            date_created: self.date_created().clone(),
            description: self.description().clone(),
            is_incomplete: *self.is_incomplet(),
            url: self.url().clone(),
            year: self.year().clone(),
            formatter: self.formatter().to_string(),
            classes: self.classes().to_py(),
            licenses: self.licenses().to_py(),
        }
    }
}
