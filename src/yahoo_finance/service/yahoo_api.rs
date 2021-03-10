use crate::fetching::model::url::Url;
use crate::fetching::service::http_client::HttpClient;
use crate::fetching::model::mime_type::MIME_APPLICATION_JSON;
use crate::fetching::model::request::Request;
use crate::prelude::*;
use crate::yahoo_finance::model::common_api::api_request::ApiRequest;
use crate::yahoo_finance::model::common_api::response::Response;
use typed_di::service::service::Service;
use std::default::Default;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YahooApiConfig {
    #[serde(rename="base_url", default = "YahooApiConfig::default_base_url")]
    base_url: Url,
}

impl YahooApiConfig {
    fn default_base_url() -> Url {
        return Url::parse("https://query2.finance.yahoo.com").unwrap();
    }
}

impl Default for YahooApiConfig {
    fn default() -> Self {
        return YahooApiConfig {
            base_url: Self::default_base_url(),
        }
    }
}

#[derive(new)]
pub struct YahooApi {
    config: YahooApiConfig,
    http_client: Service<HttpClient>,
}

impl YahooApi {
    pub async fn send(&self, request: impl ApiRequest) -> Result<Response, Failure> {
        let url = request.create_api_url(&self.config.base_url)?;
        let request = Request::get(url);
        let request = request.with_mime_type(MIME_APPLICATION_JSON);
        let response = self.http_client.send(request).await?;
        let response: Response = response.json().await?;
        return Ok(response);
    }
}