use crate::fetching::service::http_client::{HttpClient, FileDownloadRequest};
use crate::fetching::model::url::Url;
use crate::sec_gov::model::company_index_request::CompanyIndexRequest;
use crate::sec_gov::model::edgar_file::EdgarFile;
use crate::sec_gov::repository::edgar_cache::EdgarCache;
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
    edgar_cache: Service<EdgarCache>,
}

impl EdgarApi {
    pub async fn fetch_company_index(&self, request: &CompanyIndexRequest) -> Result<EdgarFile, Failure> {
        let relative_url = request.relative_url();
        let url = self
            .config
            .base_url
            .join(relative_url.as_str())?;
        if request.is_cacheable() {
            if let Some(cached_file) = self.edgar_cache.find(&relative_url).await? {
                return Ok(cached_file);
            }
        }
        let request = FileDownloadRequest::new(
            url,
            vec![MIME_APPLICATION_OCTET_STREAM, MIME_TEXT_PLAIN,],
        );
        let file = self.http_client.fetch_file(request).await?;
        self.edgar_cache.replace(&relative_url, &file).await?;
        let file = self.edgar_cache.get(&relative_url).await?;
        return Ok(file);
    }
}