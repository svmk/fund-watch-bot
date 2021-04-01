use crate::telegram::model::action_id::ActionId;
use crate::telegram::model::action_type::ActionType;
use crate::telegram::model::outgoing_message_id::OutgoingMessageId;
use crate::telegram::model::action_route::ActionRoute;
use crate::market::fund_report::model::fund::Fund;
use crate::market::fund_report::model::fund_id::FundId;
use crate::repository::model::entity::Entity;

#[derive(Debug)]
pub enum FundInfoActionDecision {
    Subscribe,
    Unsubscribe,
    FundReportList,
    FundChangeList,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundInfoAction {
    #[serde(rename="outgoing_message_id")]
    outgoing_message_id: OutgoingMessageId,
    #[serde(rename="action_id")]
    action_id: ActionId,
    #[serde(rename="fund")]
    fund: Fund,
    #[serde(rename="subscribe_route")]
    subscribe_route: ActionRoute,
    #[serde(rename="unsubscribe_route")]
    unsubscribe_route: ActionRoute,
    #[serde(rename="fund_report_list_route")]
    fund_report_list_route: ActionRoute,
    #[serde(rename="fund_change_list_route")]
    fund_change_list_route: ActionRoute,
    #[serde(rename="is_subscribed")]
    is_subscribed: bool,
}

impl FundInfoAction {
    pub fn new(fund: Fund, subscriptions: &[FundId]) -> FundInfoAction {
        let action_id = ActionId::new(ActionType::FUND_INFO);
        let is_subscribed = subscriptions.contains(fund.get_fund_id());
        return FundInfoAction {
            outgoing_message_id: OutgoingMessageId::new(),
            action_id: action_id.clone(),
            fund,
            fund_report_list_route: ActionRoute::new(action_id.clone()),
            subscribe_route: ActionRoute::new(action_id.clone()),
            unsubscribe_route: ActionRoute::new(action_id.clone()),
            fund_change_list_route: ActionRoute::new(action_id),
            is_subscribed,
        }
    }

    pub fn update_subscription(&mut self, subscriptions: &[FundId]) {
        let is_subscribed = subscriptions.contains(self.fund.get_fund_id());
        self.is_subscribed = is_subscribed;
    }

    pub fn get_outgoing_message_id(&self) -> &OutgoingMessageId {
        return &self.outgoing_message_id;
    }

    pub fn get_fund(&self) -> &Fund {
        return &self.fund;
    }

    pub fn get_fund_report_list_route(&self) -> &ActionRoute {
        return &self.fund_report_list_route;
    }

    pub fn get_fund_change_list_route(&self) -> &ActionRoute {
        return &self.fund_change_list_route;
    }

    pub fn get_subscribe_route(&self) -> &ActionRoute {
        return &self.subscribe_route;
    }

    pub fn get_unsubscribe_route(&self) -> &ActionRoute {
        return &self.unsubscribe_route;
    }

    pub fn is_subscribed(&self) -> bool {
        return self.is_subscribed;
    }

    pub fn decide(&self, action_route: &ActionRoute) -> FundInfoActionDecision {
        if &self.subscribe_route == action_route {
            return FundInfoActionDecision::Subscribe;
        } else if &self.unsubscribe_route == action_route {
            return FundInfoActionDecision::Unsubscribe;
        } else if &self.fund_report_list_route == action_route {
            return FundInfoActionDecision::FundReportList;
        } else if &self.fund_change_list_route == action_route {
            return FundInfoActionDecision::FundChangeList;
        } else {
            return FundInfoActionDecision::Unknown;
        }
    }
}

impl Entity for FundInfoAction {
    type Id = ActionId;
    fn get_entity_id(&self) -> &ActionId {
        return &self.action_id;
    }
}