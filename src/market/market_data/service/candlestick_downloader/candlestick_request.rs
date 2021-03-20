use crate::market::common::model::ticker::Ticker;
use crate::market::market_data::model::chart_period::ChartPeriod;
use crate::app::model::datetime::DateTime;

#[derive(Debug)]
pub struct CandlestickRequest {
    ticker: Ticker,
    chart_period: ChartPeriod,
}

impl CandlestickRequest {
    pub fn from_datetime(ticker: Ticker, datetime: DateTime) -> CandlestickRequest {
        return CandlestickRequest {
            ticker,
            chart_period: ChartPeriod::new(datetime.clone(), datetime),
        }
    }

    pub fn get_ticker(&self) -> &Ticker {
        return &self.ticker;
    }

    pub fn get_chart_period(&self) -> &ChartPeriod {
        return &self.chart_period;
    }
}