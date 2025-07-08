use std::collections::HashMap;

use interfaces::models::{classes::DsmClasses, ClassMapper, CustomClassMapping};
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

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyClassesMapping {
    #[pyo3(get, set)]
	classes: HashMap<String, u32>,
    #[pyo3(get, set)]
	mapping: HashMap<String, Vec<String>>,
    #[pyo3(get, set)]
	custom: Option<HashMap<String, PyCustomMap>>,
}

impl DsmToPy<PyClassesMapping> for ClassMapper {
	fn to_py(&self) -> PyClassesMapping {
		PyClassesMapping { 
			classes: self.get_classes().clone(), 
			mapping: self.get_mapping().clone(), 
			custom: self.get_custom().to_py() 
		}
	}
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyCustomMap {
    #[pyo3(get, set)]
	pub mapping: HashMap<String, String>,
}

impl DsmToPy<Option<HashMap<String, PyCustomMap>>> for Option<HashMap<String, CustomClassMapping>> {
	fn to_py(&self) -> Option<HashMap<String, PyCustomMap>> {
		match self {
			Some(maps) => {
				let mut new_map = HashMap::new();
				for (key, value) in maps {
					new_map.insert(key.clone(), PyCustomMap {
						mapping: value.mapping.clone()
					});
				}
				Some(new_map)
			},
			None => None,
		}
	}
}