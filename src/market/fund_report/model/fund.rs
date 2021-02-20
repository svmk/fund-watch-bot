use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::market::common::model::company_name::CompanyName;
use crate::repository::model::entity::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Fund {
    #[serde(rename = "fund_id")]
    fund_id: FundId,
    #[serde(rename = "company_name")]
    company_name: CompanyName,
    #[serde(rename = "last_fund_report_id")]
    last_fund_report_id: Option<DailyFundReportId>,
}

impl Fund {
    pub fn new(
        fund_id: FundId,
        company_name: CompanyName,
    ) -> Fund {
        return Fund {
            fund_id,
            company_name,
            last_fund_report_id: None,
        };
    }

    pub fn get_fund_id(&self) -> &FundId {
        return &self.fund_id;
    }

    pub fn update_last_fund_report_id(&mut self, last_fund_report_id: DailyFundReportId) {
        self.last_fund_report_id = Some(last_fund_report_id);
    }
}

impl Entity<FundId> for Fund {
    fn get_entity_id(&self) -> &FundId {
        return &self.fund_id;
    }
}