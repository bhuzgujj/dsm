mod models;

use interfaces::models::{DsmSets, Settings};
use pyo3::prelude::*;
use storage::Storage;

use crate::models::{dataset::PySet, DsmToPy};

#[pyfunction]
async fn list() -> anyhow::Result<Vec<PySet>> {
    let settings = Settings::load();
    let resp: Vec<DsmSets> = Storage::Local {
        ledger_directory: settings.get_ledger_path(),
        store_directory: settings.get_store_path(),
    }
    .list()
    .await?;
    Ok(resp.to_py())
}

#[pymodule]
fn pydsm(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(list, module)?)?;
    Ok(())
}
