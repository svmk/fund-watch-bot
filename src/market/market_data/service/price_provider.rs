use crate::market::common::model::ticker::Ticker;
use crate::market::market_data::model::ticker_price::TickerPrice;
use crate::yahoo_finance::service::yahoo_api::YahooApi;
use crate::yahoo_finance::model::chart::chart_request::ChartRequest;
use crate::yahoo_finance::model::common_api::interval::Interval;
use crate::app::model::timestamp::TimeStamp;
use crate::prelude::*;
use typed_di::service::Service;


#[derive(new)]
pub struct PriceProvider {
    yahoo_api: Service<YahooApi>,
}

impl PriceProvider {
    async fn fetch(&self, price: &mut TickerPrice) -> Result<(), Failure> {
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
        unimplemented!()
    }
}