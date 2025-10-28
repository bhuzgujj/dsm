mod commands;
mod error;
mod models;

use std::fs::create_dir_all;

use log::info;
use tauri::async_runtime::{block_on, Mutex};
use tauri::Manager;

use interfaces::models::{requires_migration, Settings, LEDGER_CURRENT_VERSION};
use interfaces::paths::log_path;
use crate::commands::*;
use crate::error::UiError;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_dialog::init())
		//.plugin(tauri_plugin_updater::Builder::new().build()) TODO
		.setup(|app| match pre_start(app) {
			Ok(_) => Ok(()),
			Err(e) => Err(Box::new(UiError::from(e))),
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

fn pre_start(app: &mut tauri::App) -> anyhow::Result<()> {
	#[cfg(debug_assertions)] // only include this code on debug builds
	{
		let window = app.get_webview_window("main").unwrap();
		window.open_devtools();
		window.close_devtools();
	}
	let mut settings = Settings::load();
	bhomz::logger::bind_logger(settings.log_level(), Some(log_path()))?;
	create_dir_all(settings.ledger_path())?;
	create_dir_all(settings.store_path())?;
	if requires_migration(settings.ledger_version()) {
		info!("Migrate to version {LEDGER_CURRENT_VERSION}");
		block_on(storage::migrate(&settings))?;
		settings.update_ledger_version();
		settings.save()?;
	}
	app.manage(Mutex::new(settings));
	Ok(())
}
