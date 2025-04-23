use std::time::Duration;

use crate::curseforge::error::CurseForgeResult;
use crate::curseforge::schemas::*;

use super::super::api::{CategoriesApi, FilesApi, GamesApi, ModsApi};
use super::ClientBuilder;
use reqwest::blocking::Client as HttpClient;

pub struct Client {
    base_url: String,
    api_key: String,
    http_client: HttpClient,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn new(base_url: &str, api_key: &str) -> Self {
        let mut http_client_builder = HttpClient::builder();
        http_client_builder = http_client_builder.connect_timeout(Duration::from_millis(3000));
        let http_client = http_client_builder.build().unwrap();
        Self {
            base_url: String::from(base_url),
            api_key: String::from(api_key),
            http_client,
        }
    }
}

impl CategoriesApi for Client {
    fn get_categories(
        &self,
        game_id: u32,
        class_id: Option<u32>,
        classes_only: Option<bool>,
    ) -> CurseForgeResult<ListResponse<Category>> {
        let mut params: Vec<String> = vec![];
        params.push(format!("gameId={}", game_id));
        if let Some(class_id) = class_id {
            params.push(format!("classId={}", class_id));
        }
        if let Some(classes_only) = classes_only {
            params.push(format!("classesOnly={}", classes_only));
        }
        let query = format!("?{}", params.join("&"));
        let url = format!("{}/v1/categories{}", self.base_url, query);
        let resp = self.http_client.get(url).send().unwrap();
        let resp_body: ListResponse<Category> = resp.json().unwrap();
        Ok(resp_body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let client = Client::builder().build();
    }
}
