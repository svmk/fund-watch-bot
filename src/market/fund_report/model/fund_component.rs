use crate::market::common::model::company_id::CompanyId;
use crate::market::common::model::share::Share;
use crate::market::common::model::original_price::OriginalPrice;
use crate::market::fund_report::model::weight::Weight;
use crate::market::fund_report::model::fund_component_share::FundComponentShare;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundComponent {
    #[serde(rename = "company_id")]
    company_id: CompanyId,
    #[serde(rename = "fund_component_share")]
    fund_component_share: FundComponentShare,
}

impl FundComponent {
    pub fn new(
        company_id: CompanyId,
        share: Share,
        price: Option<OriginalPrice>,
        weight: Weight,
    ) -> FundComponent {
        let fund_component_share = FundComponentShare::new(
            share,
            price,
            weight,
        );
        return FundComponent {
            company_id,
            fund_component_share,
        }; 
    }

    pub fn get_company_id(&self) -> &CompanyId {
        return &self.company_id;
    }

    pub fn get_share(&self) -> &FundComponentShare {
        return &self.fund_component_share;
    }
}