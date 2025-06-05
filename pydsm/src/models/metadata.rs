use std::collections::HashMap;

use interfaces::models::metadata::DsmMetaData;
use pyo3::pyclass;

use crate::models::{classes::PyClasses, licence::PyLicense, DsmToPy};

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
    classes: HashMap<u32, PyClasses>,
    licenses: HashMap<u32, PyLicense>,
}

impl DsmToPy<PyMetaData> for DsmMetaData {
    fn to_py(&self) -> PyMetaData {
        PyMetaData { 
            name: self.get_name().clone(),
            version: self.get_version().clone(),
            subset_version: self.get_subset_version().clone(), 
            contributor: self.get_contributor().clone(),
            date_created: self.get_date_created().clone(),
            description: self.get_description().clone(),
            is_incomplete: *self.is_incomplet(), 
            url: self.get_url().clone(),
            year: self.get_year().clone(),
            formatter: self.get_formatter().to_string(),
            classes: self.get_classes().to_py(),
            licenses: self.get_licenses().to_py()
        }
    }
}