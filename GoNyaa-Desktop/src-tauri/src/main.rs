mod commands;
mod domain;
mod infra;

use std::collections::HashSet;
use std::sync::Mutex;

use tauri::Manager;

use domain::mapper;
use domain::models::{AppInfo, TableData, VideoInfoItem};
use domain::settings::{AppPreferences, BrowserKind};
use infra::browser;
use infra::cache::TtlCache;
use infra::crawler_client::CrawlerClient;
use infra::errors::{AppError, AppResult};
use infra::migration;
use infra::persistence::{Persistence, StoredSettings};

const CACHE_TTL_SECONDS: u64 = 600;
const DEFAULT_CRAWLER_BASE_URL: &str = "http://api.gonyaa.szncu.club";

struct ViewedState {
    entries: HashSet<String>,
    dirty: bool,
}

pub struct AppState {
    settings: Mutex<StoredSettings>,
    crawler_client: Mutex<Option<CrawlerClient>>,
    persistence: Persistence,
    viewed: Mutex<ViewedState>,
    mgs_list: Mutex<Vec<String>>,
    list_cache: Mutex<TtlCache<Vec<VideoInfoItem>>>,
    search_cache: Mutex<TtlCache<Vec<VideoInfoItem>>>,
}

impl AppState {
    fn new(persistence: Persistence, settings: StoredSettings) -> AppResult<Self> {
        let viewed = Persistence::parse_viewed(&persistence.load_memory_text()?);
        let mgs_list = Persistence::parse_mgs_list(&persistence.load_mgs_list_text()?);
        let crawler_client = match settings.crawler_base_url.clone() {
            Some(url) => Some(CrawlerClient::new(Self::normalize_crawler_base_url(&url)?)),
            None => None,
        };

        Ok(Self {
            settings: Mutex::new(settings),
            crawler_client: Mutex::new(crawler_client),
            persistence,
            viewed: Mutex::new(ViewedState {
                entries: viewed,
                dirty: false,
            }),
            mgs_list: Mutex::new(mgs_list),
            list_cache: Mutex::new(TtlCache::new(CACHE_TTL_SECONDS)),
            search_cache: Mutex::new(TtlCache::new(CACHE_TTL_SECONDS)),
        })
    }

    fn normalize_crawler_base_url(url: &str) -> AppResult<String> {
        let normalized = url.trim().trim_end_matches('/');
        if normalized.is_empty() {
            return Err(AppError::Config("请先填写 Crawler 地址".to_string()));
        }

        let parsed = reqwest::Url::parse(normalized)
            .map_err(|error| AppError::Config(format!("Crawler 地址格式无效: {error}")))?;

        match parsed.scheme() {
            "http" | "https" => Ok(parsed.as_str().trim_end_matches('/').to_string()),
            _ => Err(AppError::Config(
                "Crawler 地址必须以 http:// 或 https:// 开头".to_string(),
            )),
        }
    }

    fn crawler_client(&self) -> AppResult<CrawlerClient> {
        self.crawler_client
            .lock()
            .expect("crawler lock")
            .clone()
            .ok_or_else(|| AppError::Config("请先配置 Crawler 地址".to_string()))
    }

    fn crawler_base_url(&self) -> Option<String> {
        self.crawler_client
            .lock()
            .expect("crawler lock")
            .as_ref()
            .map(|client| client.base_url().to_string())
    }

    fn viewed_snapshot(&self) -> HashSet<String> {
        self.viewed.lock().expect("viewed lock").entries.clone()
    }

    fn mgs_snapshot(&self) -> Vec<String> {
        self.mgs_list.lock().expect("mgs lock").clone()
    }

    fn preferences_snapshot(&self) -> AppPreferences {
        self.settings
            .lock()
            .expect("settings lock")
            .preferences
            .clone()
    }

    async fn fetch_list(&self, page: u32, sort: String) -> AppResult<TableData> {
        let key = format!("{page}:{sort}");
        if let Some(mut cached) = self.list_cache.lock().expect("list cache lock").get(&key) {
            let viewed = self.viewed_snapshot();
            mapper::apply_viewed(&mut cached, &viewed);
            mapper::apply_links(&mut cached, &self.preferences_snapshot().third_party_links);
            return Ok(TableData {
                page,
                vo_list: cached,
            });
        }

        let crawler_client = self.crawler_client()?;
        let mgs = self.mgs_snapshot();
        let viewed = self.viewed_snapshot();
        let crawler_items = crawler_client.fetch_list(page, &sort).await?;
        let information = mapper::to_information(&crawler_items, &mgs);
        let mapped = mapper::to_view_items(
            &information,
            &viewed,
            &self.preferences_snapshot().third_party_links,
        );

        self.list_cache
            .lock()
            .expect("list cache lock")
            .set(key, mapped.clone());

        Ok(TableData {
            page,
            vo_list: mapped,
        })
    }

    async fn search_list(&self, page: u32, sort: String, keyword: String) -> AppResult<TableData> {
        let key = format!("{page}:{sort}:{keyword}");
        if let Some(mut cached) = self
            .search_cache
            .lock()
            .expect("search cache lock")
            .get(&key)
        {
            let viewed = self.viewed_snapshot();
            mapper::apply_viewed(&mut cached, &viewed);
            mapper::apply_links(&mut cached, &self.preferences_snapshot().third_party_links);
            return Ok(TableData {
                page,
                vo_list: cached,
            });
        }

        let crawler_client = self.crawler_client()?;
        let mgs = self.mgs_snapshot();
        let viewed = self.viewed_snapshot();
        let crawler_items = crawler_client.search_list(page, &sort, &keyword).await?;
        let information = mapper::to_information(&crawler_items, &mgs);
        let mapped = mapper::to_view_items(
            &information,
            &viewed,
            &self.preferences_snapshot().third_party_links,
        );

        self.search_cache
            .lock()
            .expect("search cache lock")
            .set(key, mapped.clone());

        Ok(TableData {
            page,
            vo_list: mapped,
        })
    }

    fn toggle_viewed(&self, ban_go: String) -> String {
        let normalized = ban_go.trim().to_uppercase();
        let mut viewed = self.viewed.lock().expect("viewed lock");
        if viewed.entries.contains(&normalized) {
            viewed.entries.remove(&normalized);
        } else {
            viewed.entries.insert(normalized);
        }
        viewed.dirty = true;
        "change ok".to_string()
    }

    fn save_memory(&self) -> AppResult<String> {
        let mut viewed = self.viewed.lock().expect("viewed lock");
        if !viewed.dirty {
            return Ok("no changes to save".to_string());
        }

        let content = Persistence::serialize_viewed(&viewed.entries);
        self.persistence.save_memory_text(&content)?;
        viewed.dirty = false;
        Ok("save ok".to_string())
    }

    fn clear_cache(&self) -> String {
        self.list_cache.lock().expect("list cache lock").clear();
        self.search_cache.lock().expect("search cache lock").clear();
        "clear ok".to_string()
    }

    fn get_mgs_list(&self) -> Vec<String> {
        self.mgs_snapshot()
    }

    fn save_mgs_list(&self, list: Vec<String>) -> AppResult<String> {
        let mut normalized = Vec::new();
        let mut seen = HashSet::new();
        for item in list {
            let value = item.trim().to_uppercase();
            if !value.is_empty() && seen.insert(value.clone()) {
                normalized.push(value);
            }
        }

        let content = Persistence::serialize_mgs_list(&normalized);
        self.persistence.save_mgs_list_text(&content)?;
        *self.mgs_list.lock().expect("mgs lock") = normalized;
        self.clear_cache();
        Ok("save ok".to_string())
    }

    fn save_app_settings(&self, url: String, preferences: AppPreferences) -> AppResult<AppInfo> {
        let normalized = Self::normalize_crawler_base_url(&url)?;
        let preferences = preferences.normalize().map_err(AppError::Config)?;
        if preferences.browser.kind != BrowserKind::Default {
            browser::resolve_executable(&preferences.browser)?;
        }
        let crawler_changed;
        {
            let mut settings = self.settings.lock().expect("settings lock");
            crawler_changed = settings.crawler_base_url.as_deref() != Some(normalized.as_str());
            let updated = StoredSettings {
                crawler_base_url: Some(normalized.clone()),
                preferences,
            };
            self.persistence.save_settings(&updated)?;
            *settings = updated;
            if crawler_changed {
                *self.crawler_client.lock().expect("crawler lock") =
                    Some(CrawlerClient::new(normalized));
            }
        }
        if crawler_changed {
            self.clear_cache();
        }
        Ok(self.get_app_info())
    }

    fn viewed_export_content(&self) -> String {
        let viewed = self.viewed_snapshot();
        Persistence::serialize_viewed(&viewed)
    }

    fn import_viewed_content(&self, content: &str, append: bool) -> usize {
        let imported = Persistence::parse_viewed(content);
        let mut viewed = self.viewed.lock().expect("viewed lock");
        if !append {
            viewed.entries.clear();
        }
        let before = viewed.entries.len();
        viewed.entries.extend(imported);
        let added = viewed.entries.len().saturating_sub(before);
        viewed.dirty = true;
        added
    }

    fn get_app_info(&self) -> AppInfo {
        let crawler_base_url = self.crawler_base_url();
        AppInfo {
            data_dir: self.persistence.data_dir().display().to_string(),
            memory_path: self.persistence.memory_path().display().to_string(),
            mgs_list_path: self.persistence.mgs_list_path().display().to_string(),
            crawler_base_url: crawler_base_url.clone().unwrap_or_default(),
            crawler_configured: crawler_base_url.is_some(),
            preferences: self.preferences_snapshot(),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let persistence = Persistence::initialize(app.handle())?;
            migration::ensure_data_files(&persistence)?;
            let mut settings = persistence.load_settings()?;
            if settings.crawler_base_url.is_none() {
                settings.crawler_base_url = Some(
                    std::env::var("GONYAA_CRAWLER_BASE_URL")
                        .unwrap_or_else(|_| DEFAULT_CRAWLER_BASE_URL.to_string()),
                );
                persistence.save_settings(&settings)?;
            }

            let state = AppState::new(persistence, settings)?;
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list::fetch_list,
            commands::search::search_list,
            commands::viewed::toggle_viewed,
            commands::viewed::save_memory,
            commands::mgs::get_mgs_list,
            commands::mgs::save_mgs_list,
            commands::import_export::export_viewed,
            commands::import_export::import_viewed,
            commands::system::clear_cache,
            commands::system::open_external,
            commands::system::open_magnet,
            commands::system::open_data_dir,
            commands::system::get_app_info,
            commands::system::save_app_settings,
            commands::system::pick_browser_executable,
            commands::system::get_default_preferences,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn settings_survive_restart_and_update_both_caches_without_network() {
        let directory = std::env::temp_dir().join(format!(
            "gonyaa-settings-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let persistence = Persistence::in_directory(directory.clone()).unwrap();
        persistence.save_memory_text("ABC-123;").unwrap();
        persistence.save_mgs_list_text("").unwrap();
        let settings = StoredSettings {
            crawler_base_url: Some("http://127.0.0.1:1".into()),
            ..Default::default()
        };
        let state = AppState::new(persistence, settings).unwrap();
        let info = domain::models::VideoInformation {
            fan_hao: "ABC-123".into(),
            view_link: "https://example.com/official".into(),
            size: String::new(),
            date: String::new(),
            title: String::new(),
            up_cnt: String::new(),
            down_cnt: String::new(),
            fin_cnt: String::new(),
            magnet_link: String::new(),
        };
        let rows = mapper::to_view_items(
            &[info],
            &state.viewed_snapshot(),
            &state.preferences_snapshot().third_party_links,
        );
        state
            .list_cache
            .lock()
            .unwrap()
            .set("1:uploading".into(), rows.clone());
        state
            .search_cache
            .lock()
            .unwrap()
            .set("1:uploading:ABC".into(), rows);

        let mut preferences = AppPreferences::default();
        preferences.third_party_links.primary.name = "Custom Search".into();
        preferences.third_party_links.primary.url_template = "https://example.com/?q={id}".into();
        // Use a real executable path, without launching it, to verify browser settings persistence.
        preferences.browser.kind = BrowserKind::Chrome;
        preferences.browser.private_mode = true;
        preferences.browser.executable_path =
            std::env::current_exe().unwrap().display().to_string();
        state
            .save_app_settings("http://127.0.0.1:1/".into(), preferences.clone())
            .unwrap();
        let invalid = AppPreferences {
            browser: domain::settings::BrowserSettings {
                kind: BrowserKind::Default,
                private_mode: true,
                executable_path: String::new(),
            },
            ..preferences
        };
        assert!(state
            .save_app_settings("http://127.0.0.1:2".into(), invalid)
            .is_err());
        for table in [
            state.fetch_list(1, "uploading".into()).await.unwrap(),
            state
                .search_list(1, "uploading".into(), "ABC".into())
                .await
                .unwrap(),
        ] {
            assert_eq!(
                table.vo_list[0].view_link2,
                "https://example.com/?q=ABC-123"
            );
            assert_eq!(table.vo_list[0].view_link, "https://example.com/official");
            assert!(table.vo_list[0].viewed);
        }

        let persistence = Persistence::in_directory(directory.clone()).unwrap();
        let restored =
            AppState::new(persistence, state.persistence.load_settings().unwrap()).unwrap();
        assert_eq!(
            restored.crawler_base_url().as_deref(),
            Some("http://127.0.0.1:1")
        );
        let preferences = restored.preferences_snapshot();
        assert!(preferences.browser.private_mode);
        assert_eq!(preferences.browser.kind, BrowserKind::Chrome);
        assert_eq!(preferences.third_party_links.primary.name, "Custom Search");
        std::fs::remove_dir_all(directory).unwrap();
    }
}
