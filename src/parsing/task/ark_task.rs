use crate::prelude::*;
use typed_di::service::Service;
use crate::fetching::model::url::Url;
use crate::fetching::model::mime_type::MIME_TEXT_CSV;
use crate::fetching::service::http_client::{FileDownloadRequest};
use crate::fetching::service::http_client_factory::HttpClientFactory;
use crate::parsing::record::ark_csv_record::ArkCsvRecord;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::common::model::ticker::Ticker;
use crate::market::common::model::share::Share;
use crate::market::common::model::price::Price;
use crate::market::fund_report::model::weight::Weight;
use crate::market::fund_report::model::daily_fund_report::DailyFundReport;
use crate::market::fund_report::model::fund_component::FundComponent;
use std::str::FromStr;
use async_std::fs::File;
use futures::AsyncReadExt;
use csv::ReaderBuilder as CsvReaderBuilder;
use csv::Trim as CsvTrim;
#[derive(new)]
pub struct ArkTaskConfig {
    url: Url,
    fund_id: FundId,
}

#[derive(new)]
pub struct ArkkTask {
    http_client_factory: Service<HttpClientFactory>,
    config: ArkTaskConfig,
}

impl ArkkTask {
    pub async fn run(&self) -> Result<DailyFundReport, Failure> {
        unimplemented!()
    }
}