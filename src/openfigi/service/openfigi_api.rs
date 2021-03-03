use crate::prelude::*;
use crate::fetching::model::url::Url;
use crate::fetching::service::http_client::{HttpClient, Request};
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
use std::time::Duration;
use async_std::task::sleep;
use typed_di::service::Service;
mod find_by_id_request_body;
use self::find_by_id_request_body::FindByIdRequestBody;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenFigiApiConfig {
    #[serde(rename="base_url")]
    base_url: Url,
    #[serde(rename="request_delay")]
    request_delay: Duration,
    #[serde(rename="auth_token")]
    auth_token: Option<String>,
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

    pub async fn get_ticker_by_cusip(&self, cusip: &Cusip) -> Result<Ticker, Failure> {
        if let Some(cache_record) = self.cache_repository.find(cusip).await? {
            let ticker = cache_record.get_first_record()?.get_ticker()?;
            return Ok(ticker);
        }
        let url = self.config.base_url.join("/v2/mapping")?;
        let mut request = Request::post(url);
        if let Some(ref auth_token) = self.config.auth_token {
            request = request.with_header(Self::OPENFIGI_AUTH_HEADER, auth_token)?;
        }
        let body = FindByIdRequestBody::new_request_cusip(cusip.clone());
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
        let responses: Vec<Response<FigiRecord>> = response.json().await?;
        let mut records = Vec::with_capacity(responses.len());
        for response in responses {
            match response {
                Response::Data {data} => {
                    records.push(data);
                },
                Response::Error {error} => {
                    return Err(Failure::msg(format!("Cusip fetch error: {}", error)));
                },
            }
        }
        let cache_record = CusipCacheRecord::new(cusip.clone(), records);
        self.cache_repository.store(&cache_record).await?;
        let ticker = cache_record.get_first_record()?.get_ticker()?;
        return Ok(ticker);
    }
}