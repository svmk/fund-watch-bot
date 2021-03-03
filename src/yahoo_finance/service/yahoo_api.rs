use crate::fetching::model::url::Url;
use crate::fetching::service::http_client::{HttpClient, Request};
use crate::fetching::model::mime_type::MIME_APPLICATION_JSON;
use crate::prelude::*;
use crate::yahoo_finance::model::common_api::api_request::ApiRequest;
use crate::yahoo_finance::model::common_api::response::Response;
use typed_di::service::Service;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YahooApiConfig {
    #[serde(rename="base_url")]
    base_url: Url,
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