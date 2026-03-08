use tauri::State;

use crate::domain::models::WebResponse;
use crate::AppState;

#[tauri::command]
pub fn toggle_viewed(ban_go: String, state: State<'_, AppState>) -> Result<WebResponse<String>, String> {
    Ok(WebResponse::ok(state.toggle_viewed(ban_go)))
}

#[tauri::command]
pub fn save_memory(state: State<'_, AppState>) -> Result<WebResponse<String>, String> {
    state
        .save_memory()
        .map(WebResponse::ok)
        .map_err(|error| error.to_string())
}
