use std::path::PathBuf;

use interfaces::models::Settings;
use serializer::DataForm;
use storage::Storage;
use tauri::async_runtime::Mutex;
use tauri::{Error, State};

use crate::models::DatasetFormat;

#[tauri::command]
pub async fn store_raw_set(
    state: State<'_, Mutex<Settings>>,
    name: String,
    version: String,
    formats: String,
    path: String,
) -> Result<(), Error> {
    let state = state.lock().await;
    let storage = Storage::Local {
        ledger_directory: state.get_ledger_path(),
        store_directory: state.get_store_path(),
        action_log_limit: state.get_action_count(),
    };
    let path = PathBuf::from(path.clone());
    let formatter: DataForm = DatasetFormat::from(formats).into();
    let datasets = formatter.read(
        &path,
        Some(name.clone()),
        version.clone(),
        state.interpreters(),
    )?;

    for dataset in datasets {
        storage.store(&dataset, &path).await?;
        storage.ledge(&dataset).await?;
    }
    Ok(())
}
