use crate::fetching::service::http_client::HttpClient;
use crate::fetching::model::url::Url;
use crate::sec_gov::model::company_index_request::CompanyIndexRequest;
use crate::prelude::*;
use typed_di::service::Service;
use std::path::PathBuf;

#[derive(Debug)]
pub struct EdgarRepositoryConfig {
    base_url: Url,
    cache_dir: PathBuf,
}

impl EdgarRepositoryConfig {
    pub fn new(
        base_url: Url,
        cache_dir: PathBuf,
    ) -> EdgarRepositoryConfig {
        return EdgarRepositoryConfig {
            base_url,
            cache_dir,
        };
    }
}

#[derive(new, Debug)]
pub struct EdgarRepository {
    config: EdgarRepositoryConfig,
    http_client: Service<HttpClient>,
}

impl EdgarRepository {
    pub fn fetch_company_index(&self, request: &CompanyIndexRequest) -> Result<(), Failure> {
        unimplemented!()
    }
}