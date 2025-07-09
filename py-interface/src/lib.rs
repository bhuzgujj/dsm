mod models;

use interfaces::{log_err, logger};
use interfaces::models::{ClassMapper, DsmSets, MergedSet, Settings};
use pyo3::prelude::*;
use serializer::DataForm;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use storage::{add_merge_link_to, Storage};

use crate::models::merged_set::PyMergedSet;
use crate::models::{dataset::PySet, DsmToPy};

fn init() -> anyhow::Result<Settings> {
    let settings = Settings::load();
    Ok(settings)
}

async fn save_merge_set(
    name: String,
    version: String,
    storage: Storage,
    datasets: HashMap<String, (String, DsmSets)>,
    mapping: ClassMapper,
) -> anyhow::Result<()> {
    let merge_set = MergedSet::new(datasets.clone(), name.clone(), version, mapping);
    storage.ledge(&merge_set).await?;
    add_merge_link_to(&datasets, storage, merge_set).await
}

#[pyfunction]
async fn list_raw_sets() -> anyhow::Result<Vec<PySet>> {
    let settings = init()?;
    let resp: Vec<DsmSets> = Storage::Local {
        ledger_directory: settings.get_ledger_path(),
        store_directory: settings.get_store_path(),
    }
    .list()
    .await?;
    Ok(resp.to_py())
}

#[pyfunction]
async fn list_merged_sets() -> anyhow::Result<Vec<PyMergedSet>> {
    let settings = init()?;
    let resp: Vec<MergedSet> = Storage::Local {
        ledger_directory: settings.get_ledger_path(),
        store_directory: settings.get_store_path(),
    }
    .list()
    .await?;
    Ok(resp.to_py())
}

#[pyfunction]
async fn store(name: String, version: String, setform: String, path: String) -> anyhow::Result<()> {
    let settings = init()?;
    let formatter: DataForm = match setform.as_str() {
        "yolo-1-1" => DataForm::Yolo1_1,
        "coco-1-0" => DataForm::Coco1_0,
        other => DataForm::Custom(other.to_string()),
    };
    let path = PathBuf::from(path);
    let datasets = formatter.read(&path, Some(name.to_string()), version.to_string())?;
    let storage = Storage::Local {
        ledger_directory: settings.get_ledger_path(),
        store_directory: settings.get_store_path(),
    };
    for dataset in datasets.iter() {
        storage.store(dataset, &path).await?;
        storage.ledge(dataset).await?;
    }
    Ok(())
}

#[pyfunction]
async fn new_merge(
    name: String,
    version: String,
    mapping_path: String,
    dataset_to_add: Vec<(String, String, String)>,
) -> anyhow::Result<()> {
    let settings = init()?;
    let storage = Storage::Local {
        ledger_directory: settings.get_ledger_path(),
        store_directory: settings.get_store_path(),
    };
    let mut datasets = HashMap::new();
    let path = PathBuf::from(mapping_path);
    let mapping = ClassMapper::read_from_file(&path)?;
    for (name, version, group) in dataset_to_add.iter() {
        let dsm: DsmSets = storage
            .read(name.clone(), version.clone())
            .await?
            .expect(format!("Cannot read dataset {name} {version}").as_str());
        let key = dsm.get_keyed_name();
        if datasets.contains_key(&key) {
            return Err(anyhow::anyhow!("Duplicate dataset name for {name}"));
        }
        datasets.insert(key, (group.clone(), dsm));
    }

    save_merge_set(name, version, storage, datasets, mapping).await?;
    Ok(())
}

#[pyfunction]
async fn merge_on(
    name: String,
    base_version: String,
    new_name: Option<String>,
    new_version: String,
    mapping_path: Option<String>,
    dataset_to_add: Vec<(String, String, String)>,
) -> anyhow::Result<()> {
    let settings = init()?;
    let storage = Storage::Local {
        ledger_directory: settings.get_ledger_path(),
        store_directory: settings.get_store_path(),
    };
    let previous_merged_set: MergedSet = storage
        .read(name.clone(), base_version.clone())
        .await?
        .expect(format!("Cannot read dataset {name} {base_version}").as_str());
    let mut datasets = HashMap::new();
    for (group, dsms) in previous_merged_set.get_datasets_include() {
        for dsm in dsms {
            datasets.insert(dsm.get_keyed_name(), (group.clone(), dsm.clone()));
        }
    }
    for (name, version, group) in dataset_to_add.iter() {
        let dsm: DsmSets = storage
            .read(name.clone(), version.clone())
            .await?
            .expect(format!("Cannot read dataset {name} {version}").as_str());
        let key = dsm.get_keyed_name();
        if datasets.contains_key(&key) {
            return Err(anyhow::anyhow!("Duplicate dataset name for {name}"));
        }
        datasets.insert(key, (group.clone(), dsm));
    }
    let mapping = match mapping_path {
        None => previous_merged_set.get_class_mapping().clone(),
        Some(mapping) => ClassMapper::read_from_file(Path::new(&mapping))?,
    };
    save_merge_set(
        new_name.unwrap_or(name),
        new_version,
        storage,
        datasets,
        mapping,
    )
    .await?;
    Ok(())
}

#[pyfunction]
async fn generate(name: String, version: String, path: String, format: String) -> anyhow::Result<()> {
    let settings = init()?;
    if name.is_empty() {
        return log_err!("Require at least one dataset");
    }
    let formatter: DataForm = match format.as_str() {
        "yolo-1-1" => DataForm::Yolo1_1,
        "coco-1-0" => DataForm::Coco1_0,
        other => DataForm::Custom(other.to_string()),
    };
    let storage = Storage::Local {
        ledger_directory: settings.get_ledger_path(),
        store_directory: settings.get_store_path(),
    };
    let datasets = storage
        .read(name.clone(), version.clone())
        .await?;
    let (new_set, image_rel_path_mapping) = if let Some(dsm_set) = datasets {
        (dsm_set, None)
    } else {
        let merged_set: MergedSet = match storage
            .read(name.clone(), version.clone())
            .await?
        {
            Some(val) => val,
            None => return log_err!("Could not find datasets".to_string()),
        };

        merged_set.to_dataset(formatter.to_data_form())?
    };
    let path = PathBuf::from(&path);
    formatter.write(
        &path,
        &settings.get_store_path(),
        &new_set,
        image_rel_path_mapping,
    )?;
    Ok(())
}

#[pymodule]
fn pyidsm(module: &Bound<'_, PyModule>) -> PyResult<()> {
    let settings = init()?;
    logger::bind_logger(&settings)?;
    create_dir_all(settings.get_ledger_path())?;
    create_dir_all(settings.get_store_path())?;
    module.add_function(wrap_pyfunction!(generate, module)?)?;
    module.add_function(wrap_pyfunction!(list_raw_sets, module)?)?;
    module.add_function(wrap_pyfunction!(list_merged_sets, module)?)?;
    module.add_function(wrap_pyfunction!(store, module)?)?;
    module.add_function(wrap_pyfunction!(new_merge, module)?)?;
    module.add_function(wrap_pyfunction!(merge_on, module)?)?;
    Ok(())
}
