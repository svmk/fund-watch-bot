use crate::telegram::model::outgoing_message_id::OutgoingMessageId;
use crate::telegram::model::action_id::ActionId;
use crate::telegram::model::action_type::ActionType;
use crate::telegram::model::action_route::ActionRoute;
use crate::market::common::model::company_name::CompanyName;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;

#[derive(Debug, Serialize, Deserialize)]
pub struct FundRecord {
    #[serde(rename="route_subscribe")]
    route_subscribe: ActionRoute,
    #[serde(rename="route_unsubscribe")]
    route_unsubscribe: ActionRoute,
    #[serde(rename="outgoing_message_id")]
    outgoing_message_id: OutgoingMessageId,
    #[serde(rename="fund_id")]
    fund_id: FundId,
    #[serde(rename="company_name")]
    company_name: CompanyName,
    #[serde(rename="is_subscribed")]
    is_subscribed: bool,
}

impl FundRecord {
    fn new(fund: &Fund, action_id: &ActionId) -> FundRecord {
        return FundRecord {
            route_subscribe: action_id.create_route(),
            route_unsubscribe: action_id.create_route(),
            outgoing_message_id: OutgoingMessageId::new(),
            fund_id: fund.get_fund_id().clone(),
            company_name: fund.get_company_name().clone(),
            is_subscribed: false,
        }
    }

    pub fn get_outgoing_message_id(&self) -> &OutgoingMessageId {
        return &self.outgoing_message_id;
    }

    pub fn get_fund_id(&self) -> &FundId {
        return &self.fund_id;
    }

    pub fn get_company_name(&self) -> &CompanyName {
        return &self.company_name;
    }

    pub fn is_subscribed(&self) -> bool {
        return self.is_subscribed;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundListAction {
    #[serde(rename="action_id")]
    action_id: ActionId,
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
        let action_id = ActionId::new(ActionType::FUND_LIST);
        let fund_records = funds
            .iter()
            .map(|fund| {
                return FundRecord::new(fund, &action_id);
            })
            .collect();
        let mut action = FundListAction {
            action_id,
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