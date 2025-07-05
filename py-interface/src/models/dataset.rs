use std::collections::HashMap;

use interfaces::models::DsmSets;
use pyo3::pyclass;

use crate::models::{entries::PyEntry, metadata::PyMetaData, DsmToPy};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PySet {
    #[pyo3(get, set)]
    metadata: PyMetaData,
    #[pyo3(get, set)]
    entries: HashMap<String, Vec<PyEntry>>,
}

impl DsmToPy<PySet> for DsmSets {
    fn to_py(&self) -> PySet {
        PySet {
            metadata: self.get_metadata().to_py(),
            entries: self.get_entries().to_py(),
        }
    }
}
