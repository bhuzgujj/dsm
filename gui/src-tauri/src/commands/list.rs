use interfaces::models::{DsmSets, MergedSet, Settings};
use log::error;
use storage::Storage;
use tauri::async_runtime::Mutex;
use tauri::{Error, State};

#[tauri::command]
pub async fn list_merged(state: State<'_, Mutex<Settings>>) -> Result<Vec<MergedSet>, Error> {
    let state = state.lock().await;
    let storage = Storage::Local {
        ledger_directory: state.get_ledger_path(),
        store_directory: state.get_store_path(),
        action_log_limit: state.get_action_count(),
    };
    Ok(storage.list().await.unwrap_or_else(|e| {
        error!("Error listing datasets: {}", e);
        Vec::new()
    }))
}

#[tauri::command]
pub async fn list_raw(state: State<'_, Mutex<Settings>>) -> Result<Vec<DsmSets>, Error> {
    let state = state.lock().await;
    let storage = Storage::Local {
        ledger_directory: state.get_ledger_path(),
        store_directory: state.get_store_path(),
        action_log_limit: state.get_action_count(),
    };
    Ok(storage.list().await.unwrap_or_else(|e| {
        error!("Error listing datasets: {}", e);
        Vec::new()
    }))
}
