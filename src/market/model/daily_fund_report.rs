use crate::market::model::fund_id::FundId;
use crate::market::model::daily_fund_report_id::DailyFundReportId;
use crate::market::model::fund_component::FundComponent;

#[derive(Debug)]
pub struct DailyFundReport {
    id: DailyFundReportId,
    fund_components: Vec<FundComponent>,
}

impl DailyFundReport {
    pub fn new(fund_id: FundId) -> DailyFundReport {
        return DailyFundReport {
            id: DailyFundReportId::new(fund_id),
            fund_components: Vec::new(),
        };
    }

    pub fn get_id(&self) -> &DailyFundReportId {
        return &self.id;
    }

    pub fn add_fund_component(&mut self, fund_component: FundComponent) {
        self.fund_components.push(fund_component);
    }

    pub fn get_fund_components(&self) -> &[FundComponent] {
        return &self.fund_components;
    }
}