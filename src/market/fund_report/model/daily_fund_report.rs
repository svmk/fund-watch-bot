use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::market::fund_report::model::fund_component::FundComponent;
use crate::repository::model::entity::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyFundReport {
    #[serde(rename="id")]
    id: DailyFundReportId,
    #[serde(rename="fund_components")]
    fund_components: Vec<FundComponent>,
}

impl DailyFundReport {
    pub fn new(id: DailyFundReportId) -> DailyFundReport {
        return DailyFundReport {
            id,
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

impl Entity<DailyFundReportId> for DailyFundReport {
    fn get_entity_id(&self) -> &DailyFundReportId {
        return &self.id;
    }
}