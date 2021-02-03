use crate::fetching::model::url::Url;
use crate::fetching::error::pubproxy_error::PubProxy;
use crate::prelude::*;
use std::str::FromStr;
use std::net::SocketAddr;
#[derive(Debug, Serialize, Deserialize)]
pub struct PubProxyRandomProxyResponse {
    #[serde(rename="data")]
    pub proxy_list: Vec<PubProxyRandomProxyResponseItem>,
}

impl PubProxyRandomProxyResponse {
    pub fn get_first_proxy(&self) -> Result<&PubProxyRandomProxyResponseItem, PubProxy> {
        if let Some(proxy) = self.proxy_list.first() {
            return Ok(proxy);
        }
        return Err(PubProxy::NotFoundInResponse);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProxyType {
    #[serde(rename="http")]
    Http,
    #[serde(rename="https")]
    Https,
    #[serde(rename="socks")]
    Socks,
}

impl ProxyType {
    pub fn to_str(&self) -> &str {
        match self {
            &ProxyType::Http => {"http"},
            &ProxyType::Https => {"https"},
            &ProxyType::Socks => {"socks"},
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubProxyRandomProxyResponseItem {
    #[serde(rename="ipPort")]
    pub ip_port: SocketAddr,
    #[serde(rename="last_checked")]
    pub last_checked: String,
    #[serde(rename="proxy_level")]
    pub proxy_level: String,
    #[serde(rename="type")]
    pub proxy_type: ProxyType,
}

impl PubProxyRandomProxyResponseItem {
    pub fn create_proxy_url(&self) -> Result<Url, Failure> {
        let url = format!("{}://{}", self.proxy_type.to_str(), self.ip_port);
        let url = Url::from_str(&url)?;
        return Ok(url);
    }
}