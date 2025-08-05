use std::collections::HashMap;

use interfaces::models::MergedSet;
use pyo3::pyclass;
use pyo3_stub_gen_derive::gen_stub_pyclass;
use crate::models::{classes::PyClassesMapping, dataset::PySet, licence::PyLicenseMapping, DsmToPy};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyMergedSet {
    #[pyo3(get, set)]
    datasets: HashMap<String, Vec<PySet>>,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    version: String,
    #[pyo3(get, set)]
    class_mapper: PyClassesMapping,
    #[pyo3(get, set)]
    license_mapper: PyLicenseMapping,
    #[pyo3(get, set)]
    date_created: String,
    #[pyo3(get, set)]
    description: String,
}

impl DsmToPy<PyMergedSet> for MergedSet {
    fn to_py(&self) -> PyMergedSet {
        PyMergedSet {
            datasets: self.datasets_include().to_py(),
            name: self.name().to_string(),
            version: self.version().clone(),
            class_mapper: self.class_mapping().to_py(),
            license_mapper: self.license_mapping().to_py(),
            date_created: self.date_created().clone(),
            description: self.description().clone(),
        }
    }
}
