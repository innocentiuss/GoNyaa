use tauri::State;

use crate::domain::models::{TableData, WebResponse};
use crate::AppState;

#[tauri::command]
pub async fn fetch_list(
    page: u32,
    sort: String,
    state: State<'_, AppState>,
) -> Result<WebResponse<TableData>, String> {
    state
        .fetch_list(page, sort)
        .await
        .map(WebResponse::ok)
        .map_err(|error| error.to_string())
}
