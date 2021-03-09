use crate::prelude::*;
use typed_di::service::service::Service;
use crate::fetching::service::http_client::HttpClient;
use crate::fetching::service::pubproxy_client::PubProxyClient;

#[derive(new)]
pub struct HttpClientFactory {
    direct_http_client: Service<HttpClient>,
    pubproxy_client: Service<PubProxyClient>,
}

impl HttpClientFactory {
    pub async fn create_proxy_connection(&self) -> Result<HttpClient, Failure> {
        let proxy_url = self
            .pubproxy_client
            .generate_proxy_url()
            .await?;
        let http_client_config = self
            .direct_http_client
            .get_config()
            .with_proxy_url(proxy_url);
        let http_client = HttpClient::new(http_client_config)?;
        return Ok(http_client);
    }
}