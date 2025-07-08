use std::collections::HashMap;

use interfaces::models::{licence::DsmLicense, LicenseMapper};
use pyo3::pyclass;

use crate::models::DsmToPy;

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyLicense {
    #[pyo3(get, set)]
	name: String,
    #[pyo3(get, set)]
	url: String,
}

impl DsmToPy<PyLicense> for DsmLicense {
	fn to_py(&self) -> PyLicense {
		PyLicense { name: self.name.clone(), url: self.url.clone() }
	}
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyLicenseMapping {
    #[pyo3(get, set)]
    licences: HashMap<u32, PyLicense>,
    #[pyo3(get, set)]
    mapping: HashMap<String, HashMap<u32, u32>>,
}

impl DsmToPy<PyLicenseMapping> for LicenseMapper {
	fn to_py(&self) -> PyLicenseMapping {
		PyLicenseMapping { 
			licences: self.get_licences().to_py(),
			mapping: self.get_mapping().clone(),
		 }
	}
}