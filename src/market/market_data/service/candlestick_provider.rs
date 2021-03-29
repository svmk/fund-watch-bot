use crate::market::{common::model::original_candlestick::OriginalCandleStick, market_data::service::candlestick_downloader::{CandlestickDownloader, CandlestickRequest}};
use crate::market::common::model::ticker::Ticker;
use crate::market::market_data::model::candlestick_report::CandlestickReport;
use crate::market::market_data::model::time_frame::TimeFrame;
use crate::market::market_data::model::split_rules::SplitRules;
use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::market_data::model::ticker_price::TickerPrice;
use crate::market::market_data::error::candlestick_fetch_error::CandlestickFetchError;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::market::market_data::model::quartal_price::QuartalPrice;
use crate::app::model::datetime::DateTime;
use crate::app::model::year_quartal::YearQuartal;
use typed_di::service::service::Service;


#[derive(new)]
pub struct CandlestickProvider {
    candlestick_downloader: Service<CandlestickDownloader>,
    ticker_price_repository: Service<RepositoryInstance<Ticker, TickerPrice>>,
    quartal_price_repository: Service<RepositoryInstance<QuartalPriceId, QuartalPrice>>,
}

impl CandlestickProvider {
    pub async fn fetch_last_candlestick(&self, ticker: Ticker, mut datetime: DateTime) -> Result<CandlestickReport, CandlestickFetchError> {
        loop {
            if let Some(report) = self.fetch_candlestick(ticker.clone(), TimeFrame::Day, datetime.clone()).await? {
                return Ok(report);
            }
            datetime = datetime.prev_timeframe(TimeFrame::Day)?;
        }
    }

    async fn fetch_candlestick(&self, ticker: Ticker, time_frame: TimeFrame, datetime: DateTime) -> Result<Option<CandlestickReport>, CandlestickFetchError> {
        // println!("Fetching `{}` `{}`", ticker, datetime);
        let request = CandlestickRequest::from_datetime(ticker.clone(), datetime.clone());
        self.candlestick_downloader.fetch_by_ticker(&request).await?;
        let ticker_price = self.ticker_price_repository.get(&ticker).await?;
        let quartal_price_id = QuartalPriceId::new(ticker.clone(), YearQuartal::from_date(datetime.to_date()));
        match time_frame {
            TimeFrame::Year => {
                let original_candlestick = ticker_price.year_candlestick(datetime.get_year());
                return create_candlestick_report(&ticker_price, original_candlestick);
            },
            TimeFrame::Month => {
                let quartal_price = self.quartal_price_repository.get(&quartal_price_id).await?;
                let original_candlestick = quartal_price.month_candlestick(datetime.get_year(), datetime.get_month());
                return create_candlestick_report(&ticker_price, original_candlestick);
            },
            TimeFrame::Day => {
                let quartal_price = self.quartal_price_repository.get(&quartal_price_id).await?;
                let original_candlestick = quartal_price.day_candlestick(datetime.to_date());
                return create_candlestick_report(&ticker_price, original_candlestick);
            },
        }
    }

    pub async fn fetch_split_rules(&self, ticker: &Ticker, datetime: &DateTime) -> Result<SplitRules, CandlestickFetchError> {
        let request = CandlestickRequest::from_datetime(ticker.clone(), datetime.clone());
        self.candlestick_downloader.fetch_by_ticker(&request).await?;
        let ticker_price = self.ticker_price_repository.get(&ticker).await?;
        let split_rules = ticker_price.get_split_rules().clone();
        return Ok(split_rules);
    }
}

fn create_candlestick_report(ticker_price: &TickerPrice, original_candlestick: Option<OriginalCandleStick>) -> Result<Option<CandlestickReport>, CandlestickFetchError> {
    if let Some(original_candlestick) = original_candlestick {
        let actual_candlestick = ticker_price.calculate_actual_candlestick(&original_candlestick)?;
        let report = CandlestickReport::new(
            original_candlestick,
            actual_candlestick,
        );
        return Ok(Some(report));
    }
    return Ok(None);
}