use super::super::api::{CategoriesApi, FilesApi, GamesApi, ModsApi};
use super::ClientBuilder;

pub struct Client {
    pub base_url: String,
    pub api_key: String,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}
