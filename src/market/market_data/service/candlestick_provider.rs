use crate::market::market_data::service::candlestick_downloader::{CandlestickDownloader, CandlestickRequest};
use crate::market::common::model::ticker::Ticker;
use crate::market::market_data::model::candlestick_report::CandlestickReport;
use crate::app::model::datetime::DateTime;
use typed_di::service::Service;
use crate::prelude::*;


#[derive(new)]
pub struct CandlestickProvider {
    candlestick_downloader: Service<CandlestickDownloader>,
}

impl CandlestickProvider {
    pub async fn fetch_historical_candlestick(&self, ticker: Ticker, datetime: DateTime) -> Result<CandlestickReport, Failure> {
        let request = CandlestickRequest::from_datetime(ticker, datetime.clone());
        let ticker_price = self.candlestick_downloader.fetch_by_ticker(&request).await?;
        
        // let report = CandlestickReport::new(datetime);
        unimplemented!()
    }
}