use crate::{market::market_data::model::quartal_price, repository::repository::repository_instance::RepositoryInstance, sec_gov::model::year_quartal::YearQuartal};
use crate::market::common::model::ticker::Ticker;
use crate::market::market_data::model::ticker_price::TickerPrice;
use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::market_data::model::quartal_price::QuartalPrice;
use crate::market::market_data::model::day_price_id::DayPriceId;
use crate::market::market_data::model::day_price::DayPrice;
use crate::market::common::model::historical_candlestick::HistoricalCandleStick;
use crate::yahoo_finance::service::yahoo_api::YahooApi;
use crate::yahoo_finance::model::chart::chart_request::ChartRequest;
use crate::yahoo_finance::model::common_api::interval::Interval;
use crate::app::model::year_quartal_iterator::YearQuartalIterator;
use crate::app::model::timestamp::TimeStamp;
use crate::app::model::date::Date;
use crate::prelude::*;
use typed_di::service::Service;
mod candlestick_request;
pub use self::candlestick_request::CandlestickRequest;

#[derive(new)]
pub struct CandlestickDownloader {
    yahoo_api: Service<YahooApi>,
    ticker_price_repository: Service<RepositoryInstance<Ticker, TickerPrice>>,
    quartal_price_repository: Service<RepositoryInstance<QuartalPriceId, QuartalPrice>>,
    daily_price_repository: Service<RepositoryInstance<DayPriceId, DayPrice>>,
}

impl CandlestickDownloader {
    pub async fn fetch_by_ticker(&self, request: &CandlestickRequest) -> Result<TickerPrice, Failure> {
        let ticker_price = self
            .ticker_price_repository
            .find(request.get_ticker()).await?;
        if let Some(ref ticker_price) = ticker_price {
            if self.is_ticker_price_cached(request, ticker_price)? {
                return Ok(ticker_price.clone());
            }
        }
        let mut ticker_price = match ticker_price {
            Some(ticker_price) => ticker_price,
            None => {
                TickerPrice::new(request.get_ticker().clone())
            },
        };
        let chart_request = ChartRequest::new(
            ticker_price.get_ticker().clone(),
            Interval::ThreeMonths,
            TimeStamp::zero(),
            TimeStamp::now(),
        );
        let chart_response = self.yahoo_api.send(chart_request).await?;
        let chart_response = chart_response.get_charts()?;
        for split in chart_response.get_splits() {
            if ticker_price.can_add_split(&split) {
                ticker_price.add_split(split)?;
            }
        }
        let quartal_candlesticks = chart_response.get_candlesticks();
        let quartal_candlesticks = ticker_price.calculate_historical_candlesticks(quartal_candlesticks)?;
        let year_quartal = YearQuartal::now();
        for quartal_candlestick in quartal_candlesticks {
            let quartal_price = self.fetch_by_quartal(request, &ticker_price, quartal_candlestick).await?;
            let is_now_quartal_price  = quartal_price.get_period() == &year_quartal;
            if is_now_quartal_price {
                ticker_price.push_incomplete_quartal_price_once(quartal_price.clone());
            }
            ticker_price.push_quartal_price_once(quartal_price);
        }
        self.ticker_price_repository.store(&ticker_price).await?;
        return Ok(ticker_price);
    }

    fn is_ticker_price_cached(&self, request: &CandlestickRequest, ticker_price: &TickerPrice) -> Result<bool, Failure> {
        let started_at = YearQuartal::from_datetime(request.get_started_at().clone());
        let ended_at = YearQuartal::from_datetime(request.get_ended_at().clone());
        let year_quartal_iterator = YearQuartalIterator::new(started_at, ended_at)?;
        for year_quartal in year_quartal_iterator {
            let quartal_price = QuartalPriceId::from_ticker_and_year_quartal(
                ticker_price.get_ticker().clone(), 
                year_quartal,
            );
            if !ticker_price.contains_quartal_price(&quartal_price) {
                return Ok(false);
            }
            if !ticker_price.contains_incomplete_quartal_price(&quartal_price) {
                return Ok(false);
            }
        }
        return Ok(true);
    }

    async fn fetch_by_quartal(&self, request: &CandlestickRequest, ticker_price: &TickerPrice, candlestick: HistoricalCandleStick) -> Result<QuartalPriceId, Failure> {
        let quartal_price_id = QuartalPriceId::from_ticker_and_date(
            request.get_ticker().clone(), 
            candlestick.get_timestamp().clone(),
        );
        if !request.need_fetch(candlestick.get_timestamp()) {
            return Ok(quartal_price_id);
        }
        let quartal_price = self
            .quartal_price_repository
            .find(&quartal_price_id).await?;
        if let Some(ref quartal_price) = quartal_price {
            if self.is_quartal_price_cached(request, quartal_price, &candlestick)? {
                return Ok(quartal_price_id);
            }
        }
        let mut quartal_price = match quartal_price {
            Some(quartal_price) => quartal_price,
            None => {
                QuartalPrice::new(quartal_price_id.clone(), candlestick.clone())
            },
        };
        let chart_request = ChartRequest::new(
            request.get_ticker().clone(),
            Interval::OneDay,
            request.get_started_at().to_timestamp(),
            request.get_ended_at().to_timestamp(),
        );
        let chart_response = self.yahoo_api.send(chart_request).await?;
        let chart_response = chart_response.get_charts()?;
        let daily_candlesticks = chart_response.get_candlesticks();
        let daily_candlesticks = ticker_price.calculate_historical_candlesticks(daily_candlesticks)?;
        let date_now = Date::today();
        for daily_candlestick in daily_candlesticks {
            let daily_price_id = self.fetch_by_day(request, ticker_price, daily_candlestick).await?;
            let is_now_daily_price = daily_price_id.get_date() == &date_now;
            if is_now_daily_price {
                quartal_price.push_incomplete_daily_price_once(daily_price_id.clone());
            }
            quartal_price.push_daily_price_once(daily_price_id);
        }
        self.quartal_price_repository.store(&quartal_price).await?;
        return Ok(quartal_price_id);
    }

    fn is_quartal_price_cached(&self, request: &CandlestickRequest, quartal_price: &QuartalPrice, candlestick: &HistoricalCandleStick) -> Result<bool, Failure> {
        if quartal_price.is_candlestick_equals(&candlestick) {
            return Ok(false);
        }
        unimplemented!()
    }

    async fn fetch_by_day(&self, request: &CandlestickRequest, ticker_price: &TickerPrice, candlestick: HistoricalCandleStick) -> Result<DayPriceId, Failure> {
        let day_price_id = DayPriceId::from_ticker_and_date(
            request.get_ticker().clone(),
            candlestick.get_timestamp().to_date(),
        );
        let day_price = self
            .daily_price_repository
            .find(&day_price_id).await?;
        if let Some(ref day_price) = day_price {
            if day_price.is_candlestick_equals(&candlestick) {
                return Ok(day_price_id);
            }
        }
        let day_price = DayPrice::new(day_price_id.clone(), candlestick);
        self.daily_price_repository.store(&day_price).await?;
        return Ok(day_price_id);
    }
}
