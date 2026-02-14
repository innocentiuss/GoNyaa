use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoInfoItemVo {
    pub date: String,
    #[serde(rename = "fanHao")]
    pub fan_hao: String,
    #[serde(rename = "downCnt")]
    pub down_cnt: String,
    #[serde(rename = "upCnt")]
    pub up_cnt: String,
    #[serde(rename = "magnetLink")]
    pub magnet_link: String,
    pub size: String,
    pub title: String,
    #[serde(rename = "finCnt")]
    pub fin_cnt: String,
    #[serde(rename = "type")] // "type" is reserved in Rust
    pub type_: String, 
    #[serde(rename = "viewLink")]
    pub view_link: String,
    #[serde(rename = "viewLink2")]
    pub view_link2: String,
    #[serde(rename = "viewLink3")]
    pub view_link3: String,
    pub viewed: bool,
    #[serde(rename = "isViewed")]
    pub is_viewed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebResponse {
    pub code: i32,
    pub msg: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebTableDataVo {
    #[serde(rename = "voList")]
    pub vo_list: Vec<VideoInfoItemVo>,
    #[serde(rename = "currentPage")]
    pub current_page: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    #[serde(rename = "backend.ip")]
    pub backend_ip: String,
    #[serde(rename = "backend.port")]
    pub backend_port: String,
}

// Default config if file missing
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            backend_ip: "43.130.243.211".to_string(), // From original code
            backend_port: "5000".to_string(),
        }
    }
}
