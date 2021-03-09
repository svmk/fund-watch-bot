use crate::market::market_data::service::candlestick_downloader::{CandlestickDownloader, CandlestickRequest};
use crate::market::common::model::ticker::Ticker;
use crate::market::market_data::model::candlestick_report::CandlestickReport;
use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::market::market_data::model::quartal_price::QuartalPrice;
use crate::market::market_data::model::day_price_id::DayPriceId;
use crate::market::market_data::model::day_price::DayPrice;
use crate::app::model::datetime::DateTime;
use typed_di::service::service::Service;
use crate::prelude::*;


#[derive(new)]
pub struct CandlestickProvider {
    candlestick_downloader: Service<CandlestickDownloader>,
    quartal_price_repository: Service<RepositoryInstance<QuartalPriceId, QuartalPrice>>,
    daily_price_repository: Service<RepositoryInstance<DayPriceId, DayPrice>>,
}

impl CandlestickProvider {
    pub async fn fetch_historical_candlestick(&self, ticker: Ticker, datetime: DateTime) -> Result<CandlestickReport, Failure> {
        let request = CandlestickRequest::from_datetime(ticker.clone(), datetime.clone());
        let _ = self.candlestick_downloader.fetch_by_ticker(&request).await?;
        let quartal_price_id  = QuartalPriceId::from_ticker_and_date(ticker.clone(), datetime.clone());
        let quartal_price = self.quartal_price_repository.get(&quartal_price_id).await?;
        let daily_price_id = DayPriceId::from_ticker_and_date(ticker, datetime.clone().to_date());
        let daily_price = self.daily_price_repository.get(&daily_price_id).await?;
        let report = CandlestickReport::new(
            datetime,
            quartal_price.get_candlestick().clone(),
            daily_price.get_candlestick().clone(),
        );
        return Ok(report);
    }
}