use crate::models::{AppConfig, VideoInfoItemVo};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

pub struct AppState {
    pub memory: Mutex<HashSet<String>>,
    pub mgs_list: Mutex<HashSet<String>>,
    pub config: AppConfig,
    pub cache: Mutex<HashMap<String, Vec<VideoInfoItemVo>>>,
    pub search_cache: Mutex<HashMap<String, Vec<VideoInfoItemVo>>>,
}

impl AppState {
    pub fn new(memory: HashSet<String>, mgs_list: HashSet<String>, config: AppConfig) -> Self {
        Self {
            memory: Mutex::new(memory),
            mgs_list: Mutex::new(mgs_list),
            config,
            cache: Mutex::new(HashMap::new()),
            search_cache: Mutex::new(HashMap::new()),
        }
    }
}
