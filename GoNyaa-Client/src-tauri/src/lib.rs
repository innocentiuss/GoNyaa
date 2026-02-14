mod api;
mod commands;
mod file_ops;
mod models;
mod state;
mod utils;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = file_ops::load_config();
    let memory = file_ops::load_memory();
    let mgs_list = file_ops::load_mgs_list();

    let state = AppState::new(memory, mgs_list, config);

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_list,
            commands::search_list,
            commands::change_viewed,
            commands::save_memory,
            commands::get_mgs_list,
            commands::save_mgs_list,
            commands::clear_cache,
            commands::import_viewed,
            commands::export_viewed,
            commands::open_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
