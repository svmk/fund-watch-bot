use crate::prelude::*;
use crate::fetching::model::url::Url;
use crate::fetching::model::request::{Request, CONTENT_TYPE};
use crate::fetching::service::http_client::HttpClient;
use crate::fetching::error::fetch_error::FetchError;
use crate::market::common::model::cusip::Cusip;
use crate::market::common::model::ticker::Ticker;
use crate::fetching::model::mime_type::MIME_APPLICATION_JSON;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::openfigi::model::figi_record::FigiRecord;
use crate::openfigi::model::response::Response;
use crate::openfigi::model::cusip_cache_record::CusipCacheRecord; 
use crate::serializer::serializer::Serializer;
use crate::serializer::service::json_serializer::JsonSerializer;
use crate::serializer::service::serializer_instance::SerializerInstance;
use std::{str::FromStr, time::Duration};
use async_std::task::sleep;
use typed_di::service::service::Service;
mod find_by_id_request_body;
use self::find_by_id_request_body::FindByIdRequestBody;
use std::default::Default;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenFigiApiConfig {
    #[serde(rename="base_url", default="OpenFigiApiConfig::default_base_url")]
    base_url: Url,
    #[serde(rename="request_delay", default="OpenFigiApiConfig::default_request_delay")]
    request_delay: Duration,
    #[serde(rename="auth_token", default)]
    auth_token: Option<String>,
}

impl OpenFigiApiConfig {
    fn default_base_url() -> Url {
        return Url::from_str("https://api.openfigi.com/").unwrap();
    }

    fn default_request_delay() -> Duration {
        return Duration::from_secs(1);
    }
}

impl Default for OpenFigiApiConfig {
    fn default() -> Self {
        return OpenFigiApiConfig {
            base_url: OpenFigiApiConfig::default_base_url(),
            request_delay: OpenFigiApiConfig::default_request_delay(),
            auth_token: None,
        }
    }
}


pub struct OpenFigiApi {
    config: OpenFigiApiConfig,
    http_client: Service<HttpClient>,
    cache_repository: Service<RepositoryInstance<Cusip, CusipCacheRecord>>,
    serializer: SerializerInstance,
}

impl OpenFigiApi {
    const OPENFIGI_AUTH_HEADER: &'static str = "X-OPENFIGI-APIKEY";
    const OPENFIGI_RETRY_STATUS_CODE: u16 = 409;
    pub fn new(
        config: OpenFigiApiConfig,
        http_client: Service<HttpClient>,
        cache_repository: Service<RepositoryInstance<Cusip, CusipCacheRecord>>,
    ) -> OpenFigiApi {
        return OpenFigiApi {
            config,
            http_client,
            cache_repository,
            serializer: JsonSerializer::new(),
        };
    }

    pub async fn get_ticker_by_cusip(&self, cusip: &Cusip) -> Result<Option<Ticker>, Failure> {
        if let Some(cache_record) = self.cache_repository.find(cusip).await? {
            return Ok(cache_record.find_ticker());
        }
        let url = self.config.base_url.join("/v2/mapping")?;
        let mut request = Request::post(url);
        if let Some(ref auth_token) = self.config.auth_token {
            request = request.with_header(Self::OPENFIGI_AUTH_HEADER, auth_token)?;
        }
        request = request.with_header(CONTENT_TYPE, MIME_APPLICATION_JSON.as_ref())?;
        let body = vec![
            FindByIdRequestBody::new_request_cusip(cusip.clone()),
            FindByIdRequestBody::new_request_cins(cusip.clone()),
        ];
        let body = self.serializer.to_vec(&body)?;
        request = request.with_mime_type(MIME_APPLICATION_JSON);
        request = request.with_body(body)?;
        let response = loop {
            let response_result = self.http_client.send(request.clone()).await;
            match response_result {
                Ok(response) => {
                    break response;
                },
                Err(error) => {
                    if let FetchError::WrongStatusCode(ref response) = error {
                        if response.status().as_u16() != Self::OPENFIGI_RETRY_STATUS_CODE {
                            return Err(error.into());
                        }
                        sleep(self.config.request_delay.clone()).await;
                    } else {
                        return Err(error.into());
                    }
                }
            }
        };
        let response = response.bytes().await?;
        let response = response.to_vec();
        let responses: Vec<Response<Vec<FigiRecord>>> = self.serializer.from_slice(&response)?;
        let mut records = Vec::with_capacity(responses.len());
        for response in responses {
            if let Ok(response_records) = response.into_result() {
                records.extend_from_slice(response_records.as_slice());
            }
        }
        let cache_record = CusipCacheRecord::new(cusip.clone(), records);
        self.cache_repository.store(&cache_record).await?;
        return Ok(cache_record.find_ticker());
    }
}