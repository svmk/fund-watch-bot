use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::market::common::model::ticker::Ticker;
use crate::market::market_data::model::ticker_price::TickerPrice;
use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::market_data::model::quartal_price::QuartalPrice;
use crate::yahoo_finance::service::yahoo_api::YahooApi;
use crate::yahoo_finance::model::chart::chart_request::ChartRequest;
use crate::yahoo_finance::model::common_api::interval::Interval;
use crate::app::model::timestamp::TimeStamp;
use crate::prelude::*;
use typed_di::service::Service;


#[derive(new)]
pub struct PriceProvider {
    yahoo_api: Service<YahooApi>,
    quartal_price_repository: RepositoryInstance<QuartalPriceId, QuartalPrice>,
}

impl PriceProvider {
    async fn fetch_by_ticker(&self, mut price: TickerPrice) -> Result<TickerPrice, Failure> {
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