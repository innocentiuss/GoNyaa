use tauri::State;

use crate::domain::models::{TableData, WebResponse};
use crate::AppState;

#[tauri::command]
pub async fn search_list(
    page: u32,
    sort: String,
    keyword: String,
    state: State<'_, AppState>,
) -> Result<WebResponse<TableData>, String> {
    state
        .search_list(page, sort, keyword)
        .await
        .map(WebResponse::ok)
        .map_err(|error| error.to_string())
}
