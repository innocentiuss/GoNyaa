use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use rfd::FileDialog;
use tauri::State;

use crate::domain::models::WebResponse;
use crate::infra::errors::AppError;
use crate::AppState;

#[tauri::command]
pub fn export_viewed(state: State<'_, AppState>) -> Result<WebResponse<String>, String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();

    let Some(path) = FileDialog::new()
        .add_filter("Text", &["txt"])
        .set_file_name(&format!("viewed_export_{timestamp}.txt"))
        .save_file()
    else {
        return Ok(WebResponse::ok("已取消导出".to_string()));
    };

    let content = state.viewed_export_content();
    fs::write(&path, content).map_err(|error| AppError::Io(error.to_string()).to_string())?;

    Ok(WebResponse::ok(format!("已导出到 {}", path.display())))
}

#[tauri::command]
pub fn import_viewed(mode: String, state: State<'_, AppState>) -> Result<WebResponse<String>, String> {
    let Some(path) = FileDialog::new().add_filter("Text", &["txt"]).pick_file() else {
        return Ok(WebResponse::ok("已取消导入".to_string()));
    };

    let content = fs::read_to_string(&path).map_err(|error| AppError::Io(error.to_string()).to_string())?;
    let append = !mode.eq_ignore_ascii_case("overwrite");
    let imported = state.import_viewed_content(&content, append);
    let save_message = state.save_memory().map_err(|error| error.to_string())?;
    Ok(WebResponse::ok(format!("导入完成，新增 {imported} 条记录，{save_message}")))
}
