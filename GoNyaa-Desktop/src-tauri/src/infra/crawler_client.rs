use std::time::Duration;

use reqwest::Client;

use crate::domain::links;
use crate::domain::models::CrawlerResponseItem;
use crate::infra::errors::{AppError, AppResult};

#[derive(Clone)]
pub struct CrawlerClient {
    base_url: String,
    client: Client,
}

impl CrawlerClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .expect("reqwest client"),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn fetch_list(&self, page: u32, sort: &str) -> AppResult<Vec<CrawlerResponseItem>> {
        self.request("app", page, sort, None).await
    }

    pub async fn search_list(&self, page: u32, sort: &str, keyword: &str) -> AppResult<Vec<CrawlerResponseItem>> {
        self.request("search", page, sort, Some(keyword)).await
    }

    async fn request(
        &self,
        route: &str,
        page: u32,
        sort: &str,
        keyword: Option<&str>,
    ) -> AppResult<Vec<CrawlerResponseItem>> {
        let mut request = self
            .client
            .get(format!("{}/{route}", self.base_url))
            .query(&[("page", page.to_string()), ("sort", links::sort_to_crawler(sort).to_string())]);

        if let Some(keyword) = keyword {
            request = request.query(&[("keyword", keyword.to_string())]);
        }

        let response = request
            .send()
            .await
            .map_err(|error| AppError::Network(error.to_string()))?
            .error_for_status()
            .map_err(|error| AppError::Network(error.to_string()))?;

        response
            .json::<Vec<CrawlerResponseItem>>()
            .await
            .map_err(|error| AppError::Deserialize(error.to_string()))
    }
}
