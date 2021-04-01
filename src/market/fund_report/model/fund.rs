use crate::market::fund_report::model::fund_id::FundId;
use crate::market::common::model::company_name::CompanyName;
use crate::repository::model::entity::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Fund {
    #[serde(rename = "fund_id")]
    fund_id: FundId,
    #[serde(rename = "company_name")]
    company_name: CompanyName,
}

impl Fund {
    pub fn new(
        fund_id: FundId,
        company_name: CompanyName,
    ) -> Fund {
        return Fund {
            fund_id,
            company_name,
        };
    }

    pub fn get_fund_id(&self) -> &FundId {
        return &self.fund_id;
    }

    pub fn get_company_name(&self) -> &CompanyName {
        return &self.company_name;
    }
}

impl Entity for Fund {
    type Id = FundId;
    fn get_entity_id(&self) -> &FundId {
        return &self.fund_id;
    }
}