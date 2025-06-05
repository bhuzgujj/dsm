use interfaces::models::classes::DsmClasses;
use pyo3::pyclass;

use crate::models::DsmToPy;

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyClasses {
    #[pyo3(get, set)]
	class: String,
    #[pyo3(get, set)]
	subclass: Option<String>,
}

impl DsmToPy<PyClasses> for DsmClasses {
	fn to_py(&self) -> PyClasses {
		PyClasses { class: self.get_classes_name().clone(), subclass: self.get_subclass().clone() }
	}
}