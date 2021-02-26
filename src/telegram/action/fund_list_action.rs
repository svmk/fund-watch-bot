use crate::market::common::model::company_name::CompanyName;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;

#[derive(Debug, Serialize, Deserialize)]
pub struct FundInfo {
    #[serde(rename="fund_id")]
    fund_id: FundId,
    #[serde(rename="company_name")]
    company_name: CompanyName,
}

impl FundInfo {
    fn from_fund(fund: &Fund) -> FundInfo {
        return FundInfo {
            fund_id: fund.get_fund_id().clone(),
            company_name: fund.get_company_name().clone(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundListAction {
    #[serde(rename="current_page_num")]
    current_page_num: usize,
    #[serde(rename="page_size")]
    page_size: usize,
    #[serde(rename="funds_count")]
    funds_count: usize,
    #[serde(rename="funds")]
    funds: Vec<FundInfo>,
}

impl FundListAction {
    const PAGE_SIZE: usize = 10;

    pub fn new(funds: &[Fund]) -> FundListAction {
        let mut action = FundListAction {
            current_page_num: 0,
            page_size: Self::PAGE_SIZE,
            funds_count: 0,
            funds: Vec::new(),
        };
        action.update_funds(funds);
        return action;
    }

    pub fn update_funds(&mut self, funds: &[Fund]) {
        self.funds_count = funds.len();
        let funds = funds
            .iter()
            .skip(self.current_page_num * self.page_size)
            .take(self.page_size)
            .map(FundInfo::from_fund);
        self.funds = funds.collect();
    }
}