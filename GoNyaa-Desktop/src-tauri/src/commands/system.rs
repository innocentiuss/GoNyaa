use open::that_detached;
use tauri::State;

use crate::domain::models::{AppInfo, WebResponse};
use crate::domain::settings::AppPreferences;
use crate::infra::browser;
use crate::infra::errors::AppError;
use crate::infra::foreground::ForegroundLease;
use crate::AppState;

#[tauri::command]
pub fn clear_cache(state: State<'_, AppState>) -> Result<WebResponse<String>, String> {
    Ok(WebResponse::ok(state.clear_cache()))
}

#[tauri::command]
pub fn open_external(
    url: String,
    keep_foreground: Option<bool>,
    window: tauri::WebviewWindow,
    state: State<'_, AppState>,
) -> Result<WebResponse<String>, String> {
    let foreground = ForegroundLease::for_open(&window, keep_foreground.unwrap_or(false));
    browser::open_url(&state.preferences_snapshot().browser, &url)
        .map_err(|error| error.to_string())?;
    foreground.release_after_launch();
    Ok(WebResponse::ok("open ok".to_string()))
}

#[tauri::command]
pub fn open_magnet(url: String) -> Result<WebResponse<String>, String> {
    ForegroundLease::cancel();
    that_detached(url.as_str()).map_err(|error| AppError::Open(error.to_string()).to_string())?;
    Ok(WebResponse::ok("open ok".to_string()))
}

#[tauri::command]
pub fn open_data_dir(state: State<'_, AppState>) -> Result<WebResponse<String>, String> {
    ForegroundLease::cancel();
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
pub fn save_app_settings(
    state: State<'_, AppState>,
    crawler_base_url: String,
    preferences: AppPreferences,
) -> Result<WebResponse<AppInfo>, String> {
    state
        .save_app_settings(crawler_base_url, preferences)
        .map(WebResponse::ok)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_default_preferences() -> WebResponse<AppPreferences> {
    WebResponse::ok(AppPreferences::default())
}

#[tauri::command]
pub fn pick_browser_executable() -> WebResponse<Option<String>> {
    let dialog = rfd::FileDialog::new().set_title("选择浏览器可执行文件");
    #[cfg(target_os = "windows")]
    let dialog = dialog.add_filter("浏览器程序", &["exe"]);
    WebResponse::ok(dialog.pick_file().map(|path| path.display().to_string()))
}
