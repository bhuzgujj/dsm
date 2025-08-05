use std::collections::HashMap;

use interfaces::models::{licence::DsmLicense, LicenseMapper};
use pyo3::pyclass;
use pyo3_stub_gen_derive::gen_stub_pyclass;
use crate::models::DsmToPy;

#[gen_stub_pyclass]
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
		PyLicense { name: self.name().to_string(), url: self.url().to_string() }
	}
}

#[gen_stub_pyclass]
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
			licences: self.licences().to_py(),
			mapping: self.mapping().clone(),
		 }
	}
}