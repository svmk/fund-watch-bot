use crate::prelude::*;
use crate::fetching::model::url::Url;
use crate::fetching::service::http_client::{HttpClient, Request};
use crate::market::model::cusip::Cusip;
use crate::market::model::ticker::Ticker;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::openfigi::model::figi_record::FigiRecord;
use crate::openfigi::model::cusip_cache_record::CusipCacheRecord; 
use crate::serializer::serializer::Serializer;
use crate::serializer::service::json_serializer::JsonSerializer;
use crate::serializer::service::serializer_instance::SerializerInstance;
use std::time::Duration;
use typed_di::service::Service;
mod find_by_id_request_body;
use self::find_by_id_request_body::FindByIdRequestBody;


#[derive(Debug)]
pub struct OpenFigiApiConfig {
    base_url: Url,
    request_delay: Duration,
    auth_token: Option<String>,
}


pub struct OpenFigiApi {
    config: OpenFigiApiConfig,
    http_client: Service<HttpClient>,
    cache_repository: RepositoryInstance<Cusip, CusipCacheRecord>,
    serializer: SerializerInstance,
}

impl OpenFigiApi {
    const OPENFIGI_AUTH_HEADER: &'static str = "X-OPENFIGI-APIKEY";

    pub fn new(
        config: OpenFigiApiConfig,
        http_client: Service<HttpClient>,
        cache_repository: RepositoryInstance<Cusip, CusipCacheRecord>,
    ) -> OpenFigiApi {
        return OpenFigiApi {
            config,
            http_client,
            cache_repository,
            serializer: JsonSerializer::new(),
        };
    }
    pub async fn get_ticker_by_cusip(&self, cusip: Cusip) -> Result<Ticker, Failure> {
        if let Some(cache_record) = self.cache_repository.find(&cusip).await? {
            let ticker = cache_record.get_record().get_ticker()?;
            return Ok(ticker);
        }
        let url = self.config.base_url.join("/v2/mapping")?;
        let mut request = Request::post(url);
        if let Some(ref auth_token) = self.config.auth_token {
            request = request.with_header(Self::OPENFIGI_AUTH_HEADER, auth_token)?;
        }
        let body = FindByIdRequestBody::new_request_cusip(cusip);
        let body = self.serializer.to_vec(&body)?;
        unimplemented!()
    }
}