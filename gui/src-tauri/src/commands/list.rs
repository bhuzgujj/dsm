use log::error;
use tauri::async_runtime::Mutex;
use tauri::{Error, State};
use interfaces::models::{DsmSets, MergedSet, Settings};
use storage::Storage;

#[tauri::command]
pub async fn list_merged(state: State<'_, Mutex<Settings>>) -> Result<Vec<MergedSet>, Error> {
	let state = state.lock().await;
	let storage = Storage::Local {
		ledger_directory: state.get_ledger_path(),
		store_directory: state.get_store_path(),
	};
	Ok(storage.list_merged().await.unwrap_or_else(|e| {
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
	};
	Ok(storage.list_raw().await.unwrap_or_else(|e| {
		error!("Error listing datasets: {}", e);
		Vec::new()
	}))
}