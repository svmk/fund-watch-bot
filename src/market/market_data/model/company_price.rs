use crate::market::common::model::company_id::CompanyId;
use crate::{app::model::{datetime::DateTime, year_quartal_iterator::YearQuartalIterator}, market::common::model::ticker::Ticker};
use crate::app::model::year_quartal::YearQuartal;
use crate::app::model::year::Year;
use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::market_data::model::split::Split;
use crate::market::market_data::model::split_rules::SplitRules;
use crate::market::market_data::model::actual_chart_period::ActualChartPeriod;
use crate::market::market_data::model::chart::Chart;
use crate::market::common::model::original_candlestick::OriginalCandleStick;
use crate::market::common::model::actual_candlestick::ActualCandleStick;
use crate::repository::model::entity::Entity;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyPrice {
    #[serde(rename = "company_id")]
    company_id: CompanyId,
    #[serde(rename = "chart")]
    chart: Chart<YearQuartal>,
    #[serde(rename = "split_rules")]
    split_rules: SplitRules,
    #[serde(rename = "actual_chart_period")]
    actual_chart_period: ActualChartPeriod,
}

impl CompanyPrice {
    pub fn new(
        company_id: CompanyId, 
    ) -> CompanyPrice {
        return CompanyPrice {
            company_id,
            chart: Chart::new(),
            split_rules: SplitRules::new(),
            actual_chart_period: ActualChartPeriod::new_uncached(),
        };
    }

    pub fn get_company_id(&self) -> &CompanyId {
        return &self.company_id;
    }

    pub fn can_add_split(&self, split: &Split) -> bool {
        return self.split_rules.can_add_split(split);
    }

    pub fn add_split(&mut self, split: Split) -> Result<(), Failure> {
        return self.split_rules.add_split(split);
    }

    pub fn get_split_rules(&self) -> &SplitRules {
        return &self.split_rules;
    }

    pub fn need_update_chart_price(&self, id: &QuartalPriceId, price: &OriginalCandleStick) -> bool {
        return self.chart.need_update_chart_price(id.get_period(), price);
    }

    pub fn update_chart_price(&mut self, id: &QuartalPriceId, price: OriginalCandleStick) {
        self.chart.update_chart_price(id.get_period(), price);
    }

    pub fn calculate_original_candlesticks(&self, actual_candlesticks: Vec<ActualCandleStick>) -> Result<Vec<OriginalCandleStick>, Failure> {
        return self.split_rules.calculate_original_candlesticks(actual_candlesticks);
    }

    pub fn calculate_actual_candlestick(&self, original_candlestick: &OriginalCandleStick) -> Result<ActualCandleStick, Failure> {
        return self.split_rules.calculate_actual_candlestick(original_candlestick);
    }

    pub fn iter_quartal_price_ids(&self) -> Result<impl Iterator<Item=QuartalPriceId>, Failure> {
        let mut result = Vec::new();
        if let Some(chart_period) = self.actual_chart_period.get_period() {
            let begin_quartal = YearQuartal::from_date(chart_period.get_start().to_date());
            let end_quartal = YearQuartal::from_date(chart_period.get_end().to_date());
            let iterator = YearQuartalIterator::new(begin_quartal, end_quartal)?;
            let iterator = iterator.map(|quartal_price_id| {
                return QuartalPriceId::new(
                    self.company_id.clone(),
                    quartal_price_id,
                );
            });
            result = iterator.collect();
        }
        return Ok(result.into_iter());
    }

    pub fn get_actual_chart_period(&self) -> &ActualChartPeriod {
        return &self.actual_chart_period;
    }

    pub fn update_chart_period(&mut self, chart_period: ActualChartPeriod) {
        self.actual_chart_period = chart_period;
    }

    pub fn year_candlestick(&self, year: Year) -> Option<OriginalCandleStick> {
        let iterator = self.chart.iter_candlesticks();
        let timestamp = DateTime::from_year_start_day(year.clone());
        let iterator = iterator.filter(|candlestick| {
            return candlestick.get_timestamp().get_year() == year;
        });
        return OriginalCandleStick::group_from_iterator(timestamp, iterator);
    }
}

impl Entity for CompanyPrice {
    type Id = CompanyId;
    fn get_entity_id(&self) -> &CompanyId {
        return &self.company_id;
    }
}