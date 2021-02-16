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
        let http_client = self.http_client_factory.create_proxy_connection().await?;
        let request = FileDownloadRequest::new(self.config.url.clone(), vec![MIME_TEXT_CSV]);
        let csv_file = http_client.fetch_file(request).await?;
        let mut data = Vec::new();
        {
            let mut file = File::open(csv_file.get_path()).await?;
            file.read_to_end(&mut data).await?;
        }
        let mut csv_reader = CsvReaderBuilder::new()
            .has_headers(true)
            .trim(CsvTrim::All)
            .from_reader(data.as_slice());
        let mut report = DailyFundReport::new(self.config.fund_id.clone());
        for record in csv_reader.deserialize() {
            let record: ArkCsvRecord = record?;
            if record.is_empty() {
                break;
            }
            let record_ticker = Ticker::from_str(&record.ticker)?;
            let record_share = Share::from_str(&record.shares)?;
            let record_price = Price::from_str(&record.market_value)?;
            let record_weight = Weight::from_str(&record.weight)?;
            let fund_component = FundComponent::new(
                record_ticker,
                record_share,
                record_price,
                record_weight,
            );
            report.add_fund_component(fund_component);
        }
        return Ok(report);
    }
}