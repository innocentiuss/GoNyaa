use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BrowserKind {
    #[default]
    Default,
    Chrome,
    Edge,
    Firefox,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BrowserSettings {
    pub kind: BrowserKind,
    pub private_mode: bool,
    pub executable_path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkProvider {
    pub name: String,
    pub url_template: String,
}

impl LinkProvider {
    pub fn render(&self, id: &str) -> String {
        // Encode the value only, preserving the template's query and fragment syntax.
        let encoded: String = id
            .bytes()
            .map(|byte| match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    (byte as char).to_string()
                }
                _ => format!("%{byte:02X}"),
            })
            .collect();
        self.url_template.replace("{id}", &encoded)
    }

    fn normalize(&mut self) -> Result<(), String> {
        self.name = self.name.trim().to_string();
        self.url_template = self.url_template.trim().to_string();
        if self.name.is_empty() || self.name.chars().count() > 24 {
            return Err("第三方链接名称需为 1–24 个字符".into());
        }
        if !self.url_template.contains("{id}") {
            return Err(format!("{} 的链接模板必须包含 {{id}}", self.name));
        }
        let without_id = self.url_template.replace("{id}", "");
        if without_id.contains(['{', '}']) {
            return Err(format!("{} 的链接模板仅支持 {{id}} 占位符", self.name));
        }
        let sample = self.render("ABC-123");
        let url = reqwest::Url::parse(&sample)
            .map_err(|_| format!("{} 的链接模板不是有效的网址", self.name))?;
        if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
            return Err(format!(
                "{} 的链接模板必须使用 http:// 或 https://",
                self.name
            ));
        }
        // A search value belongs in the URL path/query/fragment, not the authority.
        let authority = self
            .url_template
            .split("://")
            .nth(1)
            .unwrap_or("")
            .split(['/', '?', '#'])
            .next()
            .unwrap_or("");
        if authority.contains("{id}") || !url.username().is_empty() || url.password().is_some() {
            return Err(format!(
                "{} 的 {{id}} 应放在路径、查询参数或片段中",
                self.name
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ThirdPartyLinks {
    pub primary: LinkProvider,
    pub secondary: LinkProvider,
}

impl Default for ThirdPartyLinks {
    fn default() -> Self {
        Self {
            primary: LinkProvider {
                name: "JavStore".into(),
                url_template: "https://javstore.net/search?q={id}".into(),
            },
            secondary: LinkProvider {
                name: "JavArchive".into(),
                url_template: "https://javarchive.com/#gsc.tab=0&gsc.q={id}".into(),
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CompletionHighlight {
    pub enabled: bool,
    pub threshold: u32,
}

impl Default for CompletionHighlight {
    fn default() -> Self {
        Self {
            enabled: false,
            threshold: 1000,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AppPreferences {
    pub browser: BrowserSettings,
    pub third_party_links: ThirdPartyLinks,
    pub completion_highlight: CompletionHighlight,
}

impl AppPreferences {
    pub fn normalize(mut self) -> Result<Self, String> {
        self.browser.executable_path = self.browser.executable_path.trim().to_string();
        if self.browser.kind == BrowserKind::Default {
            if self.browser.private_mode {
                return Err("使用无痕模式时，请指定 Chrome、Edge 或 Firefox".into());
            }
            self.browser.executable_path.clear();
        }
        self.third_party_links.primary.normalize()?;
        self.third_party_links.secondary.normalize()?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_encoded_ids_in_queries_and_fragments() {
        let links = ThirdPartyLinks::default();
        assert_eq!(
            links.primary.render("ABC-123 &中"),
            "https://javstore.net/search?q=ABC-123%20%26%E4%B8%AD"
        );
        assert_eq!(
            links.secondary.render("ABC-123"),
            "https://javarchive.com/#gsc.tab=0&gsc.q=ABC-123"
        );
        let custom = LinkProvider {
            name: "Custom".into(),
            url_template: "https://example.com/{id}?q={id}".into(),
        };
        assert_eq!(custom.render("A/B"), "https://example.com/A%2FB?q=A%2FB");
    }

    #[test]
    fn rejects_invalid_templates_and_default_browser_private_mode() {
        for template in [
            "https://example.com/",
            "javascript:{id}",
            "file:///{id}",
            "https://{id}.example.com/",
            "https://example.com/{other}/{id}",
        ] {
            let mut preferences = AppPreferences::default();
            preferences.third_party_links.primary.url_template = template.into();
            assert!(preferences.normalize().is_err(), "accepted {template}");
        }
        let mut preferences = AppPreferences::default();
        preferences.browser.private_mode = true;
        assert!(preferences.normalize().is_err());
        assert!(AppPreferences::default().normalize().is_ok());
    }
}
