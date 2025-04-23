use super::Client;

pub struct ClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
}

const CURSE_FORGE_BASE_URL: &str = "https://api.curseforge.com";

impl ClientBuilder {
    pub fn new() -> ClientBuilder {
        ClientBuilder {
            base_url: None,
            api_key: None,
        }
    }

    pub fn base_url(mut self, base_url: &str) -> Self {
        self.base_url = Some(base_url.to_string());
        self
    }

    pub fn api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub fn build(self) -> Client {
        let base_url = self.base_url.unwrap_or(String::from(CURSE_FORGE_BASE_URL));

        let default_api_key = String::from(env!("CURSE_FORGE_API_KEY"));
        let api_key = self.api_key.unwrap_or(default_api_key);

        Client::new(&base_url, &api_key)
    }
}
