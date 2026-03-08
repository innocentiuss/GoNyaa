mod commands;
mod domain;
mod infra;

use std::collections::HashSet;
use std::sync::Mutex;

use tauri::Manager;

use domain::mapper;
use domain::models::{AppInfo, TableData, VideoInfoItem};
use infra::cache::TtlCache;
use infra::crawler_client::CrawlerClient;
use infra::errors::{AppError, AppResult};
use infra::migration;
use infra::persistence::Persistence;

const CACHE_TTL_SECONDS: u64 = 600;

struct ViewedState {
    entries: HashSet<String>,
    dirty: bool,
}

pub struct AppState {
    crawler_client: Mutex<Option<CrawlerClient>>,
    persistence: Persistence,
    viewed: Mutex<ViewedState>,
    mgs_list: Mutex<Vec<String>>,
    list_cache: Mutex<TtlCache<Vec<VideoInfoItem>>>,
    search_cache: Mutex<TtlCache<Vec<VideoInfoItem>>>,
}

impl AppState {
    fn new(persistence: Persistence, crawler_base_url: Option<String>) -> AppResult<Self> {
        let viewed = Persistence::parse_viewed(&persistence.load_memory_text()?);
        let mgs_list = Persistence::parse_mgs_list(&persistence.load_mgs_list_text()?);
        let crawler_client = match crawler_base_url {
            Some(url) => Some(CrawlerClient::new(Self::normalize_crawler_base_url(&url)?)),
            None => None,
        };

        Ok(Self {
            crawler_client: Mutex::new(crawler_client),
            persistence,
            viewed: Mutex::new(ViewedState { entries: viewed, dirty: false }),
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

    async fn fetch_list(&self, page: u32, sort: String) -> AppResult<TableData> {
        let key = format!("{page}:{sort}");
        if let Some(mut cached) = self.list_cache.lock().expect("list cache lock").get(&key) {
            let viewed = self.viewed_snapshot();
            mapper::apply_viewed(&mut cached, &viewed);
            return Ok(TableData { page, vo_list: cached });
        }

        let crawler_client = self.crawler_client()?;
        let mgs = self.mgs_snapshot();
        let viewed = self.viewed_snapshot();
        let crawler_items = crawler_client.fetch_list(page, &sort).await?;
        let information = mapper::to_information(&crawler_items, &mgs);
        let mapped = mapper::to_view_items(&information, &viewed);

        self.list_cache
            .lock()
            .expect("list cache lock")
            .set(key, mapped.clone());

        Ok(TableData { page, vo_list: mapped })
    }

    async fn search_list(&self, page: u32, sort: String, keyword: String) -> AppResult<TableData> {
        let key = format!("{page}:{sort}:{keyword}");
        if let Some(mut cached) = self.search_cache.lock().expect("search cache lock").get(&key) {
            let viewed = self.viewed_snapshot();
            mapper::apply_viewed(&mut cached, &viewed);
            return Ok(TableData { page, vo_list: cached });
        }

        let crawler_client = self.crawler_client()?;
        let mgs = self.mgs_snapshot();
        let viewed = self.viewed_snapshot();
        let crawler_items = crawler_client.search_list(page, &sort, &keyword).await?;
        let information = mapper::to_information(&crawler_items, &mgs);
        let mapped = mapper::to_view_items(&information, &viewed);

        self.search_cache
            .lock()
            .expect("search cache lock")
            .set(key, mapped.clone());

        Ok(TableData { page, vo_list: mapped })
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

    fn save_crawler_base_url(&self, url: String) -> AppResult<AppInfo> {
        let normalized = Self::normalize_crawler_base_url(&url)?;
        self.persistence.save_crawler_base_url(&normalized)?;
        *self.crawler_client.lock().expect("crawler lock") = Some(CrawlerClient::new(normalized));
        self.clear_cache();
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
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let persistence = Persistence::initialize(app.handle())?;
            migration::ensure_data_files(&persistence)?;
            let crawler_base_url = persistence
                .load_crawler_base_url()?
                .or_else(|| std::env::var("GONYAA_CRAWLER_BASE_URL").ok());
            let state = AppState::new(persistence, crawler_base_url)?;
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
            commands::system::save_crawler_base_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
