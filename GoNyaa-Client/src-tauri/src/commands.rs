use crate::api;
use crate::file_ops;
use crate::models::{WebResponse, WebTableDataVo};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn open_url(url: String) -> WebResponse {
    match open::that(&url) {
        Ok(_) => WebResponse { code: 200, msg: serde_json::Value::String("opened".to_string()) },
        Err(e) => WebResponse { code: 500, msg: serde_json::Value::String(e.to_string()) },
    }
}

#[tauri::command]
pub fn get_list(page: String, sort: String, state: State<AppState>) -> WebResponse {
    let key = format!("{}{}", page, sort);
    
    // 1. Check Cache
    {
        let cache = state.cache.lock().unwrap();
        if let Some(list) = cache.get(&key) {
            let memory = state.memory.lock().unwrap();
            let mut new_list = list.clone();
            for item in &mut new_list {
                item.viewed = memory.contains(&item.fan_hao);
                item.is_viewed = if item.viewed { "√".to_string() } else { "×".to_string() };
            }
            let data = WebTableDataVo {
                vo_list: new_list,
                current_page: page.parse().unwrap_or(1),
            };
            return WebResponse {
                code: 200,
                msg: serde_json::to_value(data).unwrap(),
            };
        }
    }

    // 2. Fetch from Backend
    let config = state.config.clone();
    let mgs_list = state.mgs_list.lock().unwrap().clone();
    let memory = state.memory.lock().unwrap().clone();
    
    match api::do_pull_info(&page, &sort, &config, &mgs_list, &memory) {
        Ok(list) => {
            let mut cache = state.cache.lock().unwrap();
            cache.insert(key, list.clone());
            
            let data = WebTableDataVo {
                vo_list: list,
                current_page: page.parse().unwrap_or(1),
            };
            
            WebResponse {
                code: 200,
                msg: serde_json::to_value(data).unwrap(),
            }
        }
        Err(e) => {
            WebResponse {
                code: 500,
                msg: serde_json::Value::String(e.to_string()),
            }
        }
    }
}

#[tauri::command]
pub fn search_list(keyword: String, page: String, sort: String, state: State<AppState>) -> WebResponse {
    let key = format!("{}{}{}", page, sort, keyword);
    
    {
        let cache = state.search_cache.lock().unwrap();
        if let Some(list) = cache.get(&key) {
            let memory = state.memory.lock().unwrap();
            let mut new_list = list.clone();
            for item in &mut new_list {
                item.viewed = memory.contains(&item.fan_hao);
                item.is_viewed = if item.viewed { "√".to_string() } else { "×".to_string() };
            }
            let data = WebTableDataVo {
                vo_list: new_list,
                current_page: page.parse().unwrap_or(1),
            };
            return WebResponse {
                code: 200,
                msg: serde_json::to_value(data).unwrap(),
            };
        }
    }

    let config = state.config.clone();
    let mgs_list = state.mgs_list.lock().unwrap().clone();
    let memory = state.memory.lock().unwrap().clone();

    match api::do_search(&keyword, &page, &sort, &config, &mgs_list, &memory) {
        Ok(list) => {
            let mut cache = state.search_cache.lock().unwrap();
            cache.insert(key, list.clone());
            
            let data = WebTableDataVo {
                vo_list: list,
                current_page: page.parse().unwrap_or(1),
            };
            WebResponse {
                code: 200,
                msg: serde_json::to_value(data).unwrap(),
            }
        }
        Err(e) => {
            WebResponse {
                code: 500,
                msg: serde_json::Value::String(e.to_string()),
            }
        }
    }
}

#[tauri::command]
pub fn change_viewed(ban_go: String, state: State<AppState>) -> WebResponse {
    let mut memory = state.memory.lock().unwrap();
    if memory.contains(&ban_go) {
        memory.remove(&ban_go);
    } else {
        memory.insert(ban_go);
    }
    WebResponse { code: 200, msg: serde_json::Value::String("change ok".to_string()) }
}

#[tauri::command]
pub fn save_memory(state: State<AppState>) -> WebResponse {
    let memory = state.memory.lock().unwrap();
    match file_ops::save_memory(&memory) {
        Ok(_) => WebResponse { code: 200, msg: serde_json::Value::String("save ok".to_string()) },
        Err(e) => WebResponse { code: 500, msg: serde_json::Value::String(e.to_string()) },
    }
}

#[tauri::command]
pub fn clear_cache(state: State<AppState>) -> WebResponse {
    let mut cache = state.cache.lock().unwrap();
    cache.clear();
    let mut s_cache = state.search_cache.lock().unwrap();
    s_cache.clear();
    WebResponse { code: 200, msg: serde_json::Value::String("clear ok".to_string()) }
}

#[tauri::command]
pub fn get_mgs_list(state: State<AppState>) -> WebResponse {
    let list = state.mgs_list.lock().unwrap();
    let vec: Vec<String> = list.iter().cloned().collect();
    WebResponse { code: 200, msg: serde_json::to_value(vec).unwrap() }
}

#[tauri::command]
pub fn save_mgs_list(mgs_list: Vec<String>, state: State<AppState>) -> WebResponse {
    {
        let mut list_lock = state.mgs_list.lock().unwrap();
        *list_lock = mgs_list.iter().cloned().collect();
    }
    
    let list_lock = state.mgs_list.lock().unwrap();
    match file_ops::save_mgs_list(&list_lock) {
        Ok(_) => WebResponse { code: 200, msg: serde_json::Value::String("save ok".to_string()) },
        Err(e) => WebResponse { code: 500, msg: serde_json::Value::String(e.to_string()) },
    }
}

#[tauri::command]
pub fn export_viewed(state: State<AppState>) -> WebResponse {
    let memory = state.memory.lock().unwrap();
    let vec: Vec<String> = memory.iter().cloned().collect();
    let content = vec.join(";");
    let final_content = if !content.is_empty() { format!("{};", content) } else { content };
    WebResponse { code: 200, msg: serde_json::Value::String(final_content) }
}

#[tauri::command]
pub fn import_viewed(content: String, mode: String, state: State<AppState>) -> WebResponse {
    let mut memory = state.memory.lock().unwrap();
    let append = mode != "overwrite";
    
    if !append {
        memory.clear();
    }
    
    let items: Vec<&str> = content.split(|c| c == ';' || c == '\n' || c == '\r')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
        
    let count = items.len();
    for item in items {
        memory.insert(item.to_string());
    }
    
    if let Err(e) = file_ops::save_memory(&memory) {
        return WebResponse { code: 500, msg: serde_json::Value::String(format!("Imported but save failed: {}", e)) };
    }

    let msg = format!("导入成功，新增 {} 条记录（模式：{}）", count, if append { "追加" } else { "覆盖" });
    WebResponse { code: 200, msg: serde_json::Value::String(msg) }
}
