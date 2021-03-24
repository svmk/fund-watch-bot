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
    #[serde(rename="subscribe_action")]
    subscribe_action: ActionRoute,
    #[serde(rename="unsubscribe_action")]
    unsubscribe_action: ActionRoute,
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
            subscribe_action: ActionRoute::new(action_id.clone()),
            unsubscribe_action: ActionRoute::new(action_id),
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

    pub fn get_subscribe_action(&self) -> &ActionRoute {
        return &self.subscribe_action;
    }

    pub fn get_unsubscribe_action(&self) -> &ActionRoute {
        return &self.unsubscribe_action;
    }

    pub fn is_subscribed(&self) -> bool {
        return self.is_subscribed;
    }

    pub fn decide(&self, action_route: &ActionRoute) -> FundInfoActionDecision {
        if &self.subscribe_action == action_route {
            return FundInfoActionDecision::Subscribe;
        } else if &self.unsubscribe_action == action_route {
            return FundInfoActionDecision::Unsubscribe;
        } else {
            return FundInfoActionDecision::Unknown;
        }
    }
}

impl Entity<ActionId> for FundInfoAction {
    fn get_entity_id(&self) -> &ActionId {
        return &self.action_id;
    }
}