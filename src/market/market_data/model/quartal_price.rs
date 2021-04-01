use crate::{app::model::{date::Date, datetime::DateTime, day::Day, month::Month}, sec_gov::model::year::Year};
use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::market_data::model::chart::Chart;
use crate::market::common::model::original_candlestick::OriginalCandleStick;
use crate::repository::model::entity::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuartalPrice {
    #[serde(rename="id")]
    id: QuartalPriceId,
    #[serde(rename="chart")]
    chart: Chart<Date>,
    #[serde(rename="is_actual")]
    is_actual: bool,
}

impl QuartalPrice {
    pub fn new(id: QuartalPriceId) -> QuartalPrice {
        return QuartalPrice {
            id,
            chart: Chart::new(),
            is_actual: false,
        };
    }

    pub fn get_id(&self) -> &QuartalPriceId {
        return &self.id;
    }

    pub fn is_actual(&self) -> bool {
        return self.is_actual;
    }

    pub fn update_chart_price(&mut self, date: &Date, price: OriginalCandleStick) {
        self.chart.update_chart_price(&date, price);
    }

    pub fn mark_actual(&mut self) {
        self.is_actual = true;
    }

    pub fn quartal_candlestick(&self) -> Option<OriginalCandleStick> {
        let iterator = self.chart.iter_candlesticks();
        let timestamp = self.id.get_period().get_start();
        return OriginalCandleStick::group_from_iterator(timestamp, iterator);
    }

    pub fn month_candlestick(&self, year: Year, month: Month) -> Option<OriginalCandleStick> {
        let timestamp = DateTime::ymd_start_day(year.clone(), month.clone(), Day::DAY_1);
        let iterator = self.chart.iter_candlesticks();
        let iterator = iterator.filter(|candlestick| {
            let timestamp = candlestick.get_timestamp();
            if timestamp.get_year() != year {
                return false;
            }
            if timestamp.get_month() != month {
                return false;
            }
            return true;
        });
        return OriginalCandleStick::group_from_iterator(timestamp, iterator);
    }

    pub fn day_candlestick(&self, date: Date) -> Option<OriginalCandleStick> {
        let timestamp = date.start_of_day();
        let iterator = self.chart.iter_candlesticks();
        let iterator = iterator.filter(|candlestick| {
            let timestamp = candlestick.get_timestamp();
            if timestamp.to_date() != date {
                return false;
            }
            return true;
        });
        return OriginalCandleStick::group_from_iterator(timestamp, iterator);
    }

    pub fn get_candlestick_by_date(&self, date: &Date) -> Option<&OriginalCandleStick> {
        return self.chart.get(date);
    }
}

impl Entity for QuartalPrice {
    type Id = QuartalPriceId;
    fn get_entity_id(&self) -> &QuartalPriceId {
        return &self.id;
    }
}