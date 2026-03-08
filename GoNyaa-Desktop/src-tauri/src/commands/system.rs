use open::that_detached;
use tauri::State;

use crate::domain::models::{AppInfo, WebResponse};
use crate::infra::errors::AppError;
use crate::AppState;

#[tauri::command]
pub fn clear_cache(state: State<'_, AppState>) -> Result<WebResponse<String>, String> {
    Ok(WebResponse::ok(state.clear_cache()))
}

#[tauri::command]
pub fn open_external(url: String) -> Result<WebResponse<String>, String> {
    that_detached(url.as_str())
        .map_err(|error| AppError::Open(error.to_string()).to_string())?;
    Ok(WebResponse::ok("open ok".to_string()))
}

#[tauri::command]
pub fn open_magnet(url: String) -> Result<WebResponse<String>, String> {
    that_detached(url.as_str())
        .map_err(|error| AppError::Open(error.to_string()).to_string())?;
    Ok(WebResponse::ok("open ok".to_string()))
}

#[tauri::command]
pub fn open_data_dir(state: State<'_, AppState>) -> Result<WebResponse<String>, String> {
    let info = state.get_app_info();
    that_detached(info.data_dir.as_str())
        .map_err(|error| AppError::Open(error.to_string()).to_string())?;
    Ok(WebResponse::ok(info.data_dir))
}

#[tauri::command]
pub fn get_app_info(state: State<'_, AppState>) -> Result<WebResponse<AppInfo>, String> {
    Ok(WebResponse::ok(state.get_app_info()))
}

#[tauri::command]
pub fn save_crawler_base_url(
    state: State<'_, AppState>,
    url: String,
) -> Result<WebResponse<AppInfo>, String> {
    state
        .save_crawler_base_url(url)
        .map(WebResponse::ok)
        .map_err(|error| error.to_string())
}
