use crate::models::{AppConfig, VideoInfoItemVo};
use crate::utils::{self, Type};
use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};
use std::collections::HashSet;

fn parse_item(item: &serde_json::Value, mgs_set: &HashSet<String>, memory_set: &HashSet<String>) -> VideoInfoItemVo {
    let fan_hao = item["id"].as_str().unwrap_or_default().to_uppercase();
    let type_ = utils::get_type_from_id(&fan_hao, mgs_set);

    // Base64 decode title
    let title_raw = item["title"].as_str().unwrap_or_default();
    let title = match general_purpose::STANDARD.decode(title_raw) {
        Ok(bytes) => String::from_utf8(bytes).unwrap_or_else(|_| title_raw.to_string()),
        Err(_) => title_raw.to_string(),
    };

    let viewed = memory_set.contains(&fan_hao);
    let is_viewed = if viewed { "√".to_string() } else { "×".to_string() };

    // Build links before consuming type_
    let view_link = utils::get_preview_link(&fan_hao, &type_);
    let view_link2 = utils::get_jav_store_link(&fan_hao);
    let view_link3 = utils::get_jav_archive_link(&fan_hao);
    let type_str = match type_ {
        Type::FC2 => "FC2".to_string(),
        Type::Prestige => "Prestige".to_string(),
        Type::Normal => "Normal".to_string(),
    };

    VideoInfoItemVo {
        date: item["upload_time"].as_str().unwrap_or_default().to_string(),
        fan_hao,
        down_cnt: item["download_cnt"].as_str().unwrap_or_default().to_string(),
        up_cnt: item["upload_cnt"].as_str().unwrap_or_default().to_string(),
        magnet_link: item["magnet_url"].as_str().unwrap_or_default().to_string(),
        size: item["size"].as_str().unwrap_or_default().to_string(),
        title,
        fin_cnt: item["downloaded"].as_str().unwrap_or_default().to_string(),
        type_: type_str,
        view_link,
        view_link2,
        view_link3,
        viewed,
        is_viewed,
    }
}

pub fn do_pull_info(page: &str, sort: &str, config: &AppConfig, mgs_set: &HashSet<String>, memory_set: &HashSet<String>) -> Result<Vec<VideoInfoItemVo>> {
    let sort_trans = utils::sort_change(sort);
    let url = format!("http://{}:{}/app?page={}&sort={}", config.backend_ip, config.backend_port, page, sort_trans);
    
    let resp = reqwest::blocking::get(&url)?.text()?;
    let json_array: Vec<serde_json::Value> = serde_json::from_str(&resp)?;
    
    let list: Vec<VideoInfoItemVo> = json_array.iter()
        .map(|item| parse_item(item, mgs_set, memory_set))
        .collect();
    
    Ok(list)
}

pub fn do_search(keyword: &str, page: &str, sort: &str, config: &AppConfig, mgs_set: &HashSet<String>, memory_set: &HashSet<String>) -> Result<Vec<VideoInfoItemVo>> {
    let sort_trans = utils::sort_change(sort);
    let url = format!("http://{}:{}/search?page={}&sort={}&keyword={}", config.backend_ip, config.backend_port, page, sort_trans, keyword);
    
    let resp = reqwest::blocking::get(&url)?.text()?;
    let json_array: Vec<serde_json::Value> = serde_json::from_str(&resp)?;
    
    let list: Vec<VideoInfoItemVo> = json_array.iter()
        .map(|item| parse_item(item, mgs_set, memory_set))
        .collect();
    
    Ok(list)
}
