use crate::market::common::model::ticker::Ticker;
use crate::market::common::model::company_id::CompanyId;
use crate::market::market_data::model::chart_period::ChartPeriod;
use crate::app::model::datetime::DateTime;

#[derive(Debug)]
pub struct CandlestickRequest {
    company_id: CompanyId,
    chart_period: ChartPeriod,
}

impl CandlestickRequest {
    pub fn from_datetime(company_id: CompanyId, datetime: DateTime) -> CandlestickRequest {
        return CandlestickRequest {
            company_id,
            chart_period: ChartPeriod::new(datetime.clone(), datetime),
        }
    }

    pub fn get_company_id(&self) -> &CompanyId {
        return &self.company_id;
    }

    pub fn get_chart_period(&self) -> &ChartPeriod {
        return &self.chart_period;
    }
}