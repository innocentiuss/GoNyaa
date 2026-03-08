use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct CrawlerResponseItem {
    #[serde(rename = "download_cnt")]
    pub download_cnt: String,
    pub downloaded: String,
    pub id: String,
    #[serde(rename = "magnet_url")]
    pub magnet_url: String,
    pub size: String,
    pub title: String,
    #[serde(rename = "upload_cnt")]
    pub upload_cnt: String,
    #[serde(rename = "upload_time")]
    pub upload_time: String,
}

#[derive(Clone, Debug)]
pub enum VideoType {
    Fc2,
    Prestige,
    Normal,
}

#[derive(Clone, Debug)]
pub struct VideoInformation {
    pub fan_hao: String,
    pub view_link: String,
    pub size: String,
    pub date: String,
    pub title: String,
    pub up_cnt: String,
    pub down_cnt: String,
    pub fin_cnt: String,
    pub magnet_link: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfoItem {
    pub fan_hao: String,
    pub view_link: String,
    pub size: String,
    pub title: String,
    pub date: String,
    pub up_cnt: String,
    pub down_cnt: String,
    pub fin_cnt: String,
    pub magnet_link: String,
    pub is_viewed: String,
    pub viewed: bool,
    pub view_link2: String,
    pub view_link3: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableData {
    pub page: u32,
    pub vo_list: Vec<VideoInfoItem>,
}

#[derive(Clone, Debug, Serialize)]
pub struct WebResponse<T> {
    pub code: u16,
    pub msg: T,
}

impl<T> WebResponse<T> {
    pub fn ok(msg: T) -> Self {
        Self { code: 200, msg }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub data_dir: String,
    pub memory_path: String,
    pub mgs_list_path: String,
    pub crawler_base_url: String,
    pub crawler_configured: bool,
}
