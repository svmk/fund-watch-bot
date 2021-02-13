use crate::prelude::*;
use crate::fetching::model::url::Url;
use crate::fetching::service::http_client::HttpClient;
use crate::market::model::cusip::Cusip;
use std::time::Duration;
use typed_di::service::Service;

#[derive(Debug)]
pub struct OpenFigiApiConfig {
    base_url: Url,
    request_delay: Duration,
    auth_token: Option<String>,
}


#[derive(new, Debug)]
pub struct OpenFigiApi {
    config: OpenFigiApiConfig,
    http_client: Service<HttpClient>,
}

impl OpenFigiApi {
    pub async fn fetch_cusip(&self, cusip: &Cusip) -> Result<(), Failure> {
        unimplemented!()
    }
}