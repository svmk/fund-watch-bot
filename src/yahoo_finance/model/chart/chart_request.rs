use crate::yahoo_finance::model::common_api::interval::Interval;
use crate::yahoo_finance::model::common_api::api_request::ApiRequest;
use crate::market::common::model::ticker::Ticker;
use crate::app::model::datetime::DateTime;
use crate::fetching::model::url::Url;
use crate::prelude::*;

pub struct ChartRequest {
    interval: Interval,
    period_start_at: DateTime,
    period_end_at: DateTime,
    symbol: Ticker,
    include_divs: bool,
    include_splits: bool,
}

impl ApiRequest for ChartRequest {
    fn create_api_url(&self, base_url: &Url) -> Result<Url, Failure> {
        let mut url = format!(
            "/v8/finance/chart/{}?period1={}&period2={}&interval={}", 
            self.symbol,
            self.period_start_at,
            self.period_end_at,
            self.interval,
        );
        if self.include_divs || self.include_splits {
            let mut url_events: Vec<&str> = Vec::new();
            if self.include_divs {
                url_events.push("div");
            }
            if self.include_splits {
                url_events.push("split");
            }
            let url_events = url_events.join(",");
            url = format!("{}&events={}", url, url_events);
        }
        let url =  base_url.join(&url)?;
        return Ok(url);
    }
}