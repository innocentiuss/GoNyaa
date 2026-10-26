use std::collections::HashSet;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use crate::domain::links;
use crate::domain::models::{CrawlerResponseItem, VideoInfoItem, VideoInformation};
use crate::domain::settings::ThirdPartyLinks;

fn decode_title(value: &str) -> String {
    STANDARD
        .decode(value)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .unwrap_or_else(|| value.to_string())
}

pub fn to_information(
    items: &[CrawlerResponseItem],
    mgs_prefixes: &[String],
) -> Vec<VideoInformation> {
    items
        .iter()
        .map(|item| {
            let fan_hao = item.id.to_uppercase();
            let kind = links::detect_video_type(&fan_hao, mgs_prefixes);
            VideoInformation {
                fan_hao: fan_hao.clone(),
                view_link: links::preview_link(&fan_hao, &kind),
                size: item.size.clone(),
                date: item.upload_time.clone(),
                title: decode_title(&item.title),
                up_cnt: item.upload_cnt.clone(),
                down_cnt: item.download_cnt.clone(),
                fin_cnt: item.downloaded.clone(),
                magnet_link: item.magnet_url.clone(),
            }
        })
        .collect()
}

pub fn to_view_items(
    items: &[VideoInformation],
    viewed: &HashSet<String>,
    links: &ThirdPartyLinks,
) -> Vec<VideoInfoItem> {
    items
        .iter()
        .map(|item| {
            let is_viewed = viewed.contains(&item.fan_hao);
            VideoInfoItem {
                fan_hao: item.fan_hao.clone(),
                view_link: item.view_link.clone(),
                size: item.size.clone(),
                title: item.title.clone(),
                date: item.date.clone(),
                up_cnt: item.up_cnt.clone(),
                down_cnt: item.down_cnt.clone(),
                fin_cnt: item.fin_cnt.clone(),
                magnet_link: item.magnet_link.clone(),
                is_viewed: if is_viewed {
                    "是".into()
                } else {
                    "否".into()
                },
                viewed: is_viewed,
                view_link2: links.primary.render(&item.fan_hao),
                view_link3: links.secondary.render(&item.fan_hao),
            }
        })
        .collect()
}

pub fn apply_links(items: &mut [VideoInfoItem], links: &ThirdPartyLinks) {
    for item in items {
        item.view_link2 = links.primary.render(&item.fan_hao);
        item.view_link3 = links.secondary.render(&item.fan_hao);
    }
}

pub fn apply_viewed(items: &mut [VideoInfoItem], viewed: &HashSet<String>) {
    for item in items.iter_mut() {
        let is_viewed = viewed.contains(&item.fan_hao);
        item.viewed = is_viewed;
        item.is_viewed = if is_viewed {
            "是".into()
        } else {
            "否".into()
        };
    }
}
