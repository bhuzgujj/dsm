mod commands;
mod error;
mod models;

use std::fs::create_dir_all;

use tauri::async_runtime::Mutex;
use tauri::Manager;

use interfaces::logger;
use interfaces::models::Settings;

use crate::commands::*;
use crate::error::UiError;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        //.plugin(tauri_plugin_updater::Builder::new().build()) TODO
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            let settings = Settings::load();
            if let Err(e) = logger::bind_logger(&settings) {
                return Err(Box::new(UiError::from(e)));
            }
            if let Err(e) = create_dir_all(settings.get_ledger_path()) {
                return Err(Box::new(UiError::from(e)));
            }
            if let Err(e) = create_dir_all(settings.get_store_path()) {
                return Err(Box::new(UiError::from(e)));
            }
            app.manage(Mutex::new(settings));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_raw,
            list_merged,
            store_raw_set,
            merge_new,
            get_merged_set,
            get_raw_set,
            get_all_actions,
            generate_merged_set,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
