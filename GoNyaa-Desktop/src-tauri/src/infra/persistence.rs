use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::domain::settings::AppPreferences;
use crate::infra::errors::{AppError, AppResult};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct StoredSettings {
    pub crawler_base_url: Option<String>,
    #[serde(flatten)]
    pub preferences: AppPreferences,
}

pub struct Persistence {
    data_dir: PathBuf,
    memory_path: PathBuf,
    mgs_list_path: PathBuf,
    crawler_config_path: PathBuf,
}

impl Persistence {
    pub fn initialize(app: &AppHandle) -> AppResult<Self> {
        let data_dir = app
            .path()
            .app_data_dir()
            .map_err(|error| AppError::Path(error.to_string()))?;
        Self::in_directory(data_dir)
    }

    pub(crate) fn in_directory(data_dir: PathBuf) -> AppResult<Self> {
        fs::create_dir_all(&data_dir).map_err(|error| AppError::Io(error.to_string()))?;

        let memory_path = data_dir.join("memory.txt");
        let mgs_list_path = data_dir.join("MGSList.txt");
        let crawler_config_path = data_dir.join("settings.json");

        Ok(Self {
            data_dir,
            memory_path,
            mgs_list_path,
            crawler_config_path,
        })
    }

    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    pub fn memory_path(&self) -> &Path {
        &self.memory_path
    }

    pub fn mgs_list_path(&self) -> &Path {
        &self.mgs_list_path
    }

    pub fn load_memory_text(&self) -> AppResult<String> {
        fs::read_to_string(&self.memory_path).map_err(|error| AppError::Io(error.to_string()))
    }

    pub fn load_mgs_list_text(&self) -> AppResult<String> {
        fs::read_to_string(&self.mgs_list_path).map_err(|error| AppError::Io(error.to_string()))
    }

    pub fn save_memory_text(&self, content: &str) -> AppResult<()> {
        fs::write(&self.memory_path, content).map_err(|error| AppError::Io(error.to_string()))
    }

    pub fn save_mgs_list_text(&self, content: &str) -> AppResult<()> {
        fs::write(&self.mgs_list_path, content).map_err(|error| AppError::Io(error.to_string()))
    }

    pub fn load_settings(&self) -> AppResult<StoredSettings> {
        if !self.crawler_config_path.exists() {
            return Ok(StoredSettings::default());
        }

        let content = fs::read_to_string(&self.crawler_config_path)
            .map_err(|error| AppError::Io(error.to_string()))?;
        let mut settings: StoredSettings = serde_json::from_str(&content)
            .map_err(|error| AppError::Deserialize(error.to_string()))?;

        settings.crawler_base_url = settings
            .crawler_base_url
            .map(|value| value.trim().trim_end_matches('/').to_string())
            .filter(|value| !value.is_empty());
        Ok(settings)
    }

    pub fn save_settings(&self, settings: &StoredSettings) -> AppResult<()> {
        let content = serde_json::to_string_pretty(settings)
            .map_err(|error| AppError::Deserialize(error.to_string()))?;
        fs::write(&self.crawler_config_path, content)
            .map_err(|error| AppError::Io(error.to_string()))
    }

    pub fn parse_viewed(content: &str) -> HashSet<String> {
        content
            .split(|char| {
                char == ';' || char == '\n' || char == '\r' || char == ' ' || char == '\t'
            })
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_uppercase())
            .collect()
    }

    pub fn parse_mgs_list(content: &str) -> Vec<String> {
        let mut seen = HashSet::new();
        content
            .lines()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_uppercase())
            .filter(|value| seen.insert(value.clone()))
            .collect()
    }

    pub fn serialize_viewed(viewed: &HashSet<String>) -> String {
        if viewed.is_empty() {
            return String::new();
        }

        let mut items: Vec<_> = viewed.iter().cloned().collect();
        items.sort();
        format!("{};", items.join(";"))
    }

    pub fn serialize_mgs_list(items: &[String]) -> String {
        if items.is_empty() {
            return String::new();
        }

        format!("{}\n", items.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::settings::BrowserKind;

    #[test]
    fn old_settings_get_defaults_and_new_settings_round_trip() {
        let mut settings: StoredSettings =
            serde_json::from_str(r#"{"crawlerBaseUrl":"http://localhost:5001"}"#).unwrap();
        assert_eq!(settings.preferences.browser.kind, BrowserKind::Default);
        assert!(!settings.preferences.completion_highlight.enabled);
        assert_eq!(settings.preferences.completion_highlight.threshold, 1000);
        assert_eq!(
            settings.preferences.third_party_links.primary.name,
            "JavStore"
        );
        settings.preferences.browser.kind = BrowserKind::Firefox;
        settings.preferences.browser.private_mode = true;
        settings.preferences.completion_highlight.enabled = true;
        settings.preferences.completion_highlight.threshold = 1200;
        settings.preferences.third_party_links.primary.url_template =
            "https://example.com/{id}".into();
        let restored: StoredSettings =
            serde_json::from_str(&serde_json::to_string(&settings).unwrap()).unwrap();
        assert_eq!(
            restored.crawler_base_url.as_deref(),
            Some("http://localhost:5001")
        );
        assert!(restored.preferences.browser.private_mode);
        assert!(restored.preferences.completion_highlight.enabled);
        assert_eq!(restored.preferences.completion_highlight.threshold, 1200);
        assert_eq!(
            restored
                .preferences
                .third_party_links
                .primary
                .render("ABC-123"),
            "https://example.com/ABC-123"
        );
    }
}
