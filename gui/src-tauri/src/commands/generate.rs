use std::path::PathBuf;

use interfaces::models::{MergedSet, Settings};
use serializer::DataForm;
use storage::Storage;
use tauri::async_runtime::Mutex;
use tauri::{Error, State};

use crate::models::DatasetFormat;

#[tauri::command]
pub async fn generate_merged_set(
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
    let formatter: DataForm = DatasetFormat::from(formats).into();
    match storage.read::<MergedSet>(name, version).await? {
        Some(merged) => {
            let (new_set, image_rel_path_mapping) = merged.to_dataset(formatter.to_data_form())?;
            let path = PathBuf::from(&path);
            formatter.write(
                &path,
                &state.get_store_path(),
                &new_set,
                image_rel_path_mapping,
            )?;
        }
        None => {}
    };
    Ok(())
}
