use crate::domain::models::VideoType;

pub fn detect_video_type(fan_hao: &str, mgs_prefixes: &[String]) -> VideoType {
    if fan_hao.starts_with("FC2-PPV") {
        return VideoType::Fc2;
    }

    if mgs_prefixes.iter().any(|prefix| fan_hao.starts_with(prefix)) {
        return VideoType::Prestige;
    }

    VideoType::Normal
}

pub fn preview_link(fan_hao: &str, kind: &VideoType) -> String {
    match kind {
        VideoType::Fc2 => {
            let fc2_id = fan_hao.strip_prefix("FC2-PPV-").unwrap_or(fan_hao);
            format!("https://adult.contents.fc2.com/article/{fc2_id}/")
        }
        VideoType::Prestige => {
            format!("https://www.mgstage.com/search/cSearch.php?search_word={fan_hao}/")
        }
        VideoType::Normal => format!("https://www.dmm.co.jp/search/=/searchstr={fan_hao}/"),
    }
}

pub fn jav_store_link(fan_hao: &str) -> String {
    format!("https://javstore.net/search?q={fan_hao}")
}

pub fn jav_archive_link(fan_hao: &str) -> String {
    format!("https://javarchive.com/#gsc.tab=0&gsc.q={fan_hao}")
}

pub fn sort_to_crawler(sort: &str) -> &'static str {
    match sort {
        "downloading" => "leechers",
        "finished" => "downloads",
        _ => "seeders",
    }
}
