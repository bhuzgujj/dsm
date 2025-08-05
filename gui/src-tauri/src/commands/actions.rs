use std::collections::VecDeque;

use interfaces::models::actions::{get_actions, Action};
use interfaces::models::Settings;
use tauri::async_runtime::Mutex;
use tauri::{Error, State};

#[tauri::command]
pub async fn get_all_actions(state: State<'_, Mutex<Settings>>) -> Result<VecDeque<Action>, Error> {
    let state = state.lock().await;
    Ok(get_actions(state.action_count()))
}
