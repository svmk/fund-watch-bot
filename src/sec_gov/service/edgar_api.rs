use crate::fetching::service::http_client::{HttpClient, FileDownloadRequest};
use crate::fetching::model::url::Url;
use crate::sec_gov::model::year_quartal::YearQuartal;
use crate::sec_gov::model::year::Year;
use crate::sec_gov::model::relative_url::RelativeUrl;
use crate::sec_gov::model::company_report_ref::CompanyReportRef;
use crate::sec_gov::model::company_report_index::CompanyReportIndex;
use crate::sec_gov::model::company_report_13f::CompanyReport13F;
use crate::sec_gov::repository::edgar_cache::EdgarCache;
use crate::sec_gov::utils::read_edgar_company_index::read_edgar_company_index;
use crate::sec_gov::utils::read_edgar_company_report_13f::read_edgar_company_report_13f;
use crate::fetching::model::mime_type::{MIME_APPLICATION_OCTET_STREAM, MIME_TEXT_PLAIN};
use crate::prelude::*;
use typed_di::service::Service;
use std::default::Default;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EdgarApiConfig {
    #[serde(rename="base_url", default="EdgarApiConfig::default_base_url")]
    base_url: Url,
    #[serde(rename="retry_delay", default="EdgarApiConfig::default_retry_delay")]
    retry_delay: Option<Duration>,
}

impl EdgarApiConfig {
    fn default_base_url() -> Url {
        return Url::parse("https://www.sec.gov/").unwrap();
    }

    fn default_retry_delay() -> Option<Duration> {
        return Some(Duration::from_millis(500));
    }
}

impl Default for EdgarApiConfig {
    fn default() -> Self {
        return EdgarApiConfig {
            base_url: Self::default_base_url(),
            retry_delay: Self::default_retry_delay(),
        }
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
            "Archives/edgar/full-index/{}/{}/company.idx",
            year_quartal.get_year(),
            year_quartal.get_quartal().display_long(),
        );
        let relative_url = RelativeUrl::new(relative_url);
        let url = self
            .config
            .base_url
            .join(relative_url.as_str())?;
        println!("url = `{}`", url);
        if Self::is_year_quartal_cacheable(year_quartal) {
            if let Some(cached_file) = self.edgar_cache.find(&relative_url).await? {
                let company_index = read_edgar_company_index(cached_file).await?;
                return Ok(company_index);
            }
        }
        let mut request = FileDownloadRequest::new(
            url,
            vec![MIME_APPLICATION_OCTET_STREAM, MIME_TEXT_PLAIN,],
        );
        if let Some(retry_delay) = self.config.retry_delay {
            request = request.with_retry_delay(retry_delay.clone());
        }
        let file = self.http_client.fetch_file(request).await?;
        // {
        //     println!("file = {:?}", file);
        //     let mut buf = String::new();
        //     let _ = std::io::stdin().read_line(&mut buf).unwrap();
        // }
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

    pub async fn fetch_compoany_report_13f(&self, company_ref: &CompanyReportRef) -> Result<Option<CompanyReport13F>, Failure> {
        let relative_url = company_ref.get_relative_url();
        let url = self
            .config
            .base_url
            .join(relative_url.as_str())?;
        if let Some(cached_file) = self.edgar_cache.find(&relative_url).await? {
            let company_report = read_edgar_company_report_13f(cached_file).await?;
            return Ok(company_report);
        }
        let mut request = FileDownloadRequest::new(
            url,
            vec![MIME_APPLICATION_OCTET_STREAM, MIME_TEXT_PLAIN,],
        );
        if let Some(retry_delay) = self.config.retry_delay {
            request = request.with_retry_delay(retry_delay.clone());
        }
        let file = self.http_client.fetch_file(request).await?;
        self.edgar_cache.replace(&relative_url, &file).await?;
        let file = self.edgar_cache.get(&relative_url).await?;
        let company_report = read_edgar_company_report_13f(file).await?;
        return Ok(company_report);
    }
}