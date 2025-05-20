use interfaces::logger;
use interfaces::models::{DsmSets, MergedSet, Settings};
use std::fs::create_dir_all;
use tauri::{Error, Manager, State};
use log::error;
use tauri::async_runtime::Mutex;
use storage::Storage;

#[tauri::command]
async fn list_merged_datasets(state: State<'_, Mutex<Settings>>) -> Result<Vec<MergedSet>, Error> {
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
async fn list_raw_datasets(state: State<'_, Mutex<Settings>>) -> Result<Vec<DsmSets>, Error> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        //.plugin(tauri_plugin_updater::Builder::new().build()) TODO
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            let settings = Settings::load();
            logger::bind_logger(&settings).expect("Could not init the logger");
            create_dir_all(settings.get_ledger_path()).expect("Could not create the ledger directory");
            create_dir_all(settings.get_store_path()).expect("Could not create the store directory");
            app.manage(Mutex::new(settings));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_merged_datasets,
            list_raw_datasets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
