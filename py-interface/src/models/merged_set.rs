use interfaces::models::MergedSet;
use pyo3::pyclass;

use crate::models::{classes::PyClassesMapping, dataset::PySet, licence::PyLicenseMapping, DsmToPy};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyMergedSet {
    #[pyo3(get, set)]
    datasets: Vec<PySet>,
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
            datasets: self.get_datasets_include().to_py(),
            name: self.get_name().to_string(),
            version: self.get_version().clone(),
            class_mapper: self.get_class_mapping().to_py(),
            license_mapper: self.get_license_mapping().to_py(),
            date_created: self.get_date_created().clone(),
            description: self.get_description().clone(),
        }
    }
}
