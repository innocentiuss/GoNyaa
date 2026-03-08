use tauri::State;

use crate::domain::models::WebResponse;
use crate::AppState;

#[tauri::command]
pub fn get_mgs_list(state: State<'_, AppState>) -> Result<WebResponse<Vec<String>>, String> {
    Ok(WebResponse::ok(state.get_mgs_list()))
}

#[tauri::command]
pub fn save_mgs_list(list: Vec<String>, state: State<'_, AppState>) -> Result<WebResponse<String>, String> {
    state
        .save_mgs_list(list)
        .map(WebResponse::ok)
        .map_err(|error| error.to_string())
}
