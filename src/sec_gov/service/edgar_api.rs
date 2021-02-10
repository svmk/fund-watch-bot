use crate::fetching::service::http_client::{HttpClient, FileDownloadRequest};
use crate::fetching::model::url::Url;
use crate::sec_gov::model::company_index_request::CompanyIndexRequest;
use crate::fetching::model::mime_type::{MIME_APPLICATION_OCTET_STREAM, MIME_TEXT_PLAIN};
use crate::prelude::*;
use typed_di::service::Service;

#[derive(Debug)]
pub struct EdgarApiConfig {
    base_url: Url,
}

impl EdgarApiConfig {
    pub fn new(
        base_url: Url,
    ) -> EdgarApiConfig {
        return EdgarApiConfig {
            base_url,
        };
    }
}

#[derive(new, Debug)]
pub struct EdgarApi {
    config: EdgarApiConfig,
    http_client: Service<HttpClient>,
}

impl EdgarApi {
    pub async fn fetch_company_index(&self, request: &CompanyIndexRequest) -> Result<(), Failure> {
        let path = request.relative_path();
        let url = self
            .config
            .base_url
            .join(path.as_str())?;
        let request = FileDownloadRequest::new(
            url, 
            vec![MIME_APPLICATION_OCTET_STREAM, MIME_TEXT_PLAIN,],
        );
        let file = self.http_client.fetch_file(request).await?;
        unimplemented!()
    }
}