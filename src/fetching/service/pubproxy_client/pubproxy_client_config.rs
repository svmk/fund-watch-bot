use crate::prelude::*;
use crate::fetching::model::url::Url;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct PubProxyClientConfig {
    #[serde(rename="api_gateway")]
    api_gateway: Url,
}

impl Default for PubProxyClientConfig {
    fn default() -> Self {
        return PubProxyClientConfig {
            api_gateway: Url::from_str("http://pubproxy.com").unwrap(),
        };
    }
}

impl PubProxyClientConfig {
    pub fn create_api_random_proxy(&self) -> Result<Url, Failure> {
        let url = self.api_gateway.join("/api/proxy?limit=1")?;
        return Ok(url);
    }
}