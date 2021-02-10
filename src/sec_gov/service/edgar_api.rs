use crate::fetching::service::http_client::{HttpClient, FileDownloadRequest};
use crate::fetching::model::url::Url;
use crate::sec_gov::model::year_quartal::YearQuartal;
use crate::sec_gov::model::year::Year;
use crate::sec_gov::model::relative_url::RelativeUrl;
use crate::sec_gov::model::edgar_file::EdgarFile;
use crate::sec_gov::model::company_report_index::CompanyReportIndex;
use crate::sec_gov::repository::edgar_cache::EdgarCache;
use crate::sec_gov::utils::read_edgar_company_index::read_edgar_company_index;
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
    pub async fn fetch_company_index(&self, year_quartal: &YearQuartal) -> Result<CompanyReportIndex, Failure> {
        let relative_url = format!(
            "{}/{}.idx",
            year_quartal.get_year(),
            year_quartal.get_quartal().display_long(),
        );
        let relative_url = RelativeUrl::new(relative_url);
        let url = self
            .config
            .base_url
            .join(relative_url.as_str())?;
        if Self::is_year_quartal_cacheable(year_quartal) {
            if let Some(cached_file) = self.edgar_cache.find(&relative_url).await? {
                let company_index = read_edgar_company_index(cached_file).await?;
                return Ok(company_index);
            }
        }
        let request = FileDownloadRequest::new(
            url,
            vec![MIME_APPLICATION_OCTET_STREAM, MIME_TEXT_PLAIN,],
        );
        let file = self.http_client.fetch_file(request).await?;
        self.edgar_cache.replace(&relative_url, &file).await?;
        let file = self.edgar_cache.get(&relative_url).await?;
        let company_index = read_edgar_company_index(file).await?;
        return Ok(company_index);
    }

    fn is_year_quartal_cacheable(year_quartal: &YearQuartal) -> bool {
        if year_quartal.get_year() == &Year::now() {
            return false;
        }
        return true;
    }
}