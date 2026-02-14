use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use crate::models::AppConfig;

pub fn get_app_dir() -> PathBuf {
    // In dev, execute from src-tauri/target/debug/
    // We want config files relative to exe?
    // Or just relative to CwD?
    // User requested "same directory as exe".
    std::env::current_exe()
        .map(|p| p.parent().unwrap().to_path_buf())
        .unwrap_or_else(|_| PathBuf::from("."))
}

pub fn load_config() -> AppConfig {
    let path = get_app_dir().join("application.json");
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        AppConfig::default()
    }
}

pub fn load_memory() -> HashSet<String> {
    let path = get_app_dir().join("memory.txt");
    let content = fs::read_to_string(&path).unwrap_or_default();
    content.split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn save_memory(set: &HashSet<String>) -> anyhow::Result<()> {
    let path = get_app_dir().join("memory.txt");
    let content = set.iter().cloned().collect::<Vec<_>>().join(";");
    // Add semi-colon at end to match old format
    let final_content = if !content.is_empty() { format!("{};", content) } else { content };
    fs::write(path, final_content)?;
    Ok(())
}

pub fn load_mgs_list() -> HashSet<String> {
    let path = get_app_dir().join("MGSList.txt");
    let content = fs::read_to_string(&path).unwrap_or_default();
    content.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn save_mgs_list(set: &HashSet<String>) -> anyhow::Result<()> {
    let path = get_app_dir().join("MGSList.txt");
    // Sort slightly? No, just save.
    let content = set.iter().cloned().collect::<Vec<_>>().join("\n");
    fs::write(path, content)?;
    Ok(())
}
