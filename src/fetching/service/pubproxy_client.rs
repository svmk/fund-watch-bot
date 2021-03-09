mod pubproxy_client_config;
use self::pubproxy_client_config::PubProxyClientConfig;
use crate::fetching::service::http_client::{HttpClient, Request};
use crate::fetching::model::url::Url;
use crate::fetching::model::mime_type::MIME_APPLICATION_JSON;
use crate::fetching::record::pubproxy_random_proxy_response::PubProxyRandomProxyResponse;
use typed_di::service::service::Service;
use crate::prelude::*;


#[derive(new)]
pub struct PubProxyClient {
    config: PubProxyClientConfig,
    http_client: Service<HttpClient>,
}

impl PubProxyClient {
    pub async fn generate_proxy_url(&self) -> Result<Url, Failure> {
        let api_url = self.config.create_api_random_proxy()?;
        let response = self
            .http_client
            .send(Request::get(api_url).with_mime_type(MIME_APPLICATION_JSON))
            .await?;
        let response: PubProxyRandomProxyResponse = response.json().await?;
        let proxy = response.get_first_proxy()?;
        let proxy_url = proxy.create_proxy_url()?;
        return Ok(proxy_url);
    }
}