use std::collections::HashSet;

#[derive(PartialEq)]
pub enum Type {
    FC2,
    Prestige,
    Normal,
}

pub fn get_type_from_id(fan_hao: &str, mgs_set: &HashSet<String>) -> Type {
    if fan_hao.to_uppercase().starts_with("FC2-PPV") {
        return Type::FC2;
    }
    for s in mgs_set {
        if fan_hao.starts_with(s) {
            return Type::Prestige;
        }
    }
    Type::Normal
}

pub fn get_preview_link(fan_hao: &str, type_: &Type) -> String {
    match type_ {
        Type::FC2 => {
            // fan_hao is FC2-PPV-123456, we want 123456
            // But verify if Java logic does "FC2-PPV" stripping
            // Java: fanHao.substring(8); "FC2-PPV-" is 8 chars.
            if fan_hao.len() > 8 {
                 let fc2_id = &fan_hao[8..];
                 format!("https://adult.contents.fc2.com/article/{}/", fc2_id)
            } else {
                 format!("https://adult.contents.fc2.com/article/{}/", fan_hao)
            }
        }
        Type::Normal => {
            format!("https://www.dmm.co.jp/search/=/searchstr={}/", fan_hao)
        }
        Type::Prestige => {
            format!("https://www.mgstage.com/search/cSearch.php?search_word={}/", fan_hao)
        }
    }
}

pub fn get_jav_store_link(fan_hao: &str) -> String {
    format!("https://javstore.net/search?q={}", fan_hao)
}

pub fn get_jav_archive_link(fan_hao: &str) -> String {
    format!("https://javarchive.com/#gsc.tab=0&gsc.q={}", fan_hao)
}

pub fn sort_change(origin: &str) -> &str {
    match origin {
        "downloading" => "leechers",
        "uploading" => "seeders",
        _ => "downloads",
    }
}
