use crate::market::common::model::company_name::CompanyName;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;

#[derive(new, Debug, Serialize, Deserialize)]
pub struct FundRecord {
    #[serde(rename="fund_id")]
    fund_id: FundId,
    #[serde(rename="company_name")]
    company_name: CompanyName,
    #[serde(rename="is_subscribed")]
    is_subscribed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundListAction {
    #[serde(rename="current_page_num")]
    current_page_num: usize,
    #[serde(rename="page_size")]
    page_size: usize,
    #[serde(rename="funds_count")]
    funds_count: usize,
    #[serde(rename="fund_records")]
    fund_records: Vec<FundRecord>,
}

impl FundListAction {
    const PAGE_SIZE: usize = 10;

    pub fn new(funds: &[Fund], subscriptions: &[FundId]) -> FundListAction {
        let mut action = FundListAction {
            current_page_num: 0,
            page_size: Self::PAGE_SIZE,
            funds_count: 0,
            fund_records: Vec::new(),
        };
        action.update_funds(funds, subscriptions);
        return action;
    }

    pub fn update_funds(&mut self, funds: &[Fund], subscriptions: &[FundId]) {
        self.funds_count = funds.len();
        let funds_iter = funds
            .iter()
            .skip(self.current_page_num * self.page_size)
            .take(self.page_size);
        let mut fund_records = Vec::with_capacity(funds.len());
        for fund in funds_iter {
            let is_subscribed = subscriptions.contains(fund.get_fund_id());
            let fund_record = FundRecord::new(
                fund.get_fund_id().clone(),
                fund.get_company_name().clone(),
                is_subscribed,
            );
            fund_records.push(fund_record);
        }
        self.fund_records = fund_records;
    }
}