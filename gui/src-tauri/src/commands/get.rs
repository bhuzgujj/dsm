use interfaces::models::{DsmSets, MergedSet, Settings};
use storage::Storage;
use tauri::async_runtime::Mutex;
use tauri::{Error, State};

#[tauri::command]
pub async fn get_raw_set(
    state: State<'_, Mutex<Settings>>,
    name: String,
    version: String,
) -> Result<Option<DsmSets>, Error> {
    let state = state.lock().await;
    let storage = Storage::Local {
        ledger_directory: state.ledger_path(),
        store_directory: state.store_path(),
        action_log_limit: state.action_count(),
    };
    Ok(storage.read(name, version).await.unwrap_or(None))
}

#[tauri::command]
pub async fn get_merged_set(
    state: State<'_, Mutex<Settings>>,
    name: String,
    version: String,
) -> Result<Option<MergedSet>, Error> {
    let state = state.lock().await;
    let storage = Storage::Local {
        ledger_directory: state.ledger_path(),
        store_directory: state.store_path(),
        action_log_limit: state.action_count(),
    };
    Ok(storage.read(name, version).await.unwrap_or(None))
}
