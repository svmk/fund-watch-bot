use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::market::common::model::ticker::Ticker;
use crate::market::market_data::model::ticker_price::TickerPrice;
use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::market_data::model::quartal_price::QuartalPrice;
use crate::yahoo_finance::service::yahoo_api::YahooApi;
use crate::yahoo_finance::model::chart::chart_request::ChartRequest;
use crate::yahoo_finance::model::common_api::interval::Interval;
use crate::app::model::timestamp::TimeStamp;
use crate::app::model::datetime::DateTime;
use crate::prelude::*;
use typed_di::service::Service;
mod candlestick_request;
pub use self::candlestick_request::CandlestickRequest;

#[derive(new)]
pub struct CandlestickDownloader {
    yahoo_api: Service<YahooApi>,
    ticker_price_repository: Service<RepositoryInstance<Ticker, TickerPrice>>,
    quartal_price_repository: Service<RepositoryInstance<QuartalPriceId, QuartalPrice>>,
}

impl CandlestickDownloader {
    async fn fetch_by_ticker(&self, request: &CandlestickRequest) -> Result<TickerPrice, Failure> {
        let price = self
            .ticker_price_repository
            .find(request.get_ticker()).await?;
        let mut price = match price {
            Some(price) => price,
            None => {
                TickerPrice::new(request.get_ticker().clone())
            },
        };
        let request = ChartRequest::new(
            price.get_ticker().clone(),
            Interval::ThreeMonths,
            TimeStamp::zero(),
            TimeStamp::now(),
        );
        let response = self.yahoo_api.send(request).await?;
        let response = response.get_charts()?;
        for split in response.get_splits() {
            if price.can_add_split(&split) {
                price.add_split(split)?;
            }
        }
        let quartal_candlesticks = response.get_candlesticks();
        let quartal_candlesticks = price.calculate_historical_candlesticks(quartal_candlesticks)?;
        for quartal_candlestick in quartal_candlesticks {

        }
        unimplemented!()
    }

    // async fn fetch_by_quartal(&self, )
}