use interfaces::models::licence::DsmLicense;
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