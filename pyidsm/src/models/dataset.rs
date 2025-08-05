use std::collections::HashMap;

use interfaces::models::DsmSets;
use pyo3::pyclass;
use pyo3_stub_gen_derive::gen_stub_pyclass;
use crate::models::{entries::PyEntry, metadata::PyMetaData, DsmToPy};

#[gen_stub_pyclass]
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
            metadata: self.metadata().to_py(),
            entries: self.entries().to_py(),
        }
    }
}
