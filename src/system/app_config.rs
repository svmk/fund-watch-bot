use crate::fetching::service::http_client::HttpClientConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(rename="http_client")]
    http_client: HttpClientConfig,
}

impl AppConfig {
    pub fn get_http_client(&self) -> HttpClientConfig {
        return self.http_client.clone();
    }
}