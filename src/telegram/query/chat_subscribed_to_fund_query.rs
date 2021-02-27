use crate::market::fund_report::model::fund_id::FundId;
use crate::repository::model::query::Query;

#[derive(new, Debug)]
pub struct ChatSubscribedToFundQuery {
    fund_id: FundId,
}

impl ChatSubscribedToFundQuery {
    pub fn get_fund_id(&self) -> &FundId {
        return &self.fund_id;
    }
}

impl Query for ChatSubscribedToFundQuery {}