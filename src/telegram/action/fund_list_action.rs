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

impl FundRecord {
    fn from_fund(fund: &Fund) -> FundRecord {
        return FundRecord {
            fund_id: fund.get_fund_id().clone(),
            company_name: fund.get_company_name().clone(),
            is_subscribed: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundListAction {
    #[serde(rename="current_page_num")]
    current_page_num: usize,
    #[serde(rename="page_size")]
    page_size: usize,
    #[serde(rename="fund_records")]
    fund_records: Vec<FundRecord>,
}

impl FundListAction {
    const PAGE_SIZE: usize = 10;

    pub fn new(funds: &[Fund], subscriptions: &[FundId]) -> FundListAction {
        let fund_records = funds.iter().map(FundRecord::from_fund).collect();
        let mut action = FundListAction {
            current_page_num: 0,
            page_size: Self::PAGE_SIZE,
            fund_records,
        };
        action.update_subscriptions(subscriptions);
        return action;
    }

    pub fn update_subscriptions(&mut self, subscriptions: &[FundId]) {
        for fund_record in self.fund_records.iter_mut() {
            fund_record.is_subscribed = subscriptions.contains(&fund_record.fund_id);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&FundRecord> {
        let skip = self.current_page_num * self.page_size;
        return self.fund_records.iter().skip(skip).take(self.page_size);
    }
}