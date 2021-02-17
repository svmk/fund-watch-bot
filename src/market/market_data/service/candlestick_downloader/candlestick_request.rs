use crate::market::common::model::ticker::Ticker;
use crate::app::model::datetime::DateTime;
use crate::app::model::timestamp::TimeStamp;

pub struct CandlestickRequest {
    ticker: Ticker,
    started_at: DateTime,
    ended_at: DateTime,
}

impl CandlestickRequest {
    pub fn get_ticker(&self) -> &Ticker {
        return &self.ticker;
    }

    pub fn get_started_at(&self) -> &DateTime {
        return &self.started_at;
    }

    pub fn get_ended_at(&self) -> &DateTime {
        return &self.ended_at;
    }

    pub fn need_fetch(&self, datetime: &DateTime) -> bool {
        return &self.started_at <= datetime && datetime <= &self.ended_at;
    }
}