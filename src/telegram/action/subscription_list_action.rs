use crate::telegram::model::outgoing_message_id::OutgoingMessageId;
use crate::telegram::model::action_id::ActionId;
use crate::telegram::model::action_type::ActionType;
use crate::telegram::model::action_route::ActionRoute;
use crate::telegram::action::pager_action::{PagerAction, Page};
use crate::market::common::model::company_name::CompanyName;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::repository::model::entity::Entity;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionRecord {
    #[serde(rename="route_view")]
    route_view: ActionRoute,
    #[serde(rename="route_unsubscribe")]
    route_unsubscribe: ActionRoute,
    #[serde(rename="fund_id")]
    fund_id: FundId,
    #[serde(rename="company_name")]
    company_name: CompanyName,
    #[serde(rename="is_subscribed")]
    is_subscribed: bool,
}

impl SubscriptionRecord {
    fn new(fund: &Fund, action_id: &ActionId) -> SubscriptionRecord {
        return SubscriptionRecord {
            route_view: action_id.create_route(),
            route_unsubscribe: action_id.create_route(),
            fund_id: fund.get_fund_id().clone(),
            company_name: fund.get_company_name().clone(),
            is_subscribed: false,
        }
    }

    pub fn get_fund_id(&self) -> &FundId {
        return &self.fund_id;
    }

    pub fn get_company_name(&self) -> &CompanyName {
        return &self.company_name;
    }

    pub fn get_route_view(&self) -> &ActionRoute {
        return &self.route_view;
    }

    pub fn get_route_unsubscribe(&self) -> &ActionRoute {
        return &self.route_unsubscribe;
    }

    pub fn is_subscribed(&self) -> bool {
        return self.is_subscribed;
    }
}


#[derive(Debug)]
pub enum SubscriptionListActionDecision {
    View(FundId),
    Unsubscribe(FundId),
    SelectPage(Page),
    UnknownRoute,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionListAction {
    #[serde(rename="action_id")]
    action_id: ActionId,
    #[serde(rename="pager")]
    pager: PagerAction,
    #[serde(rename="fund_records")]
    fund_records: Vec<SubscriptionRecord>,
    #[serde(rename="outgoing_message_id")]
    outgoing_message_id: OutgoingMessageId,
}

impl SubscriptionListAction {
    fn new(action_type: ActionType, funds: &[Fund], subscriptions: &[FundId]) -> SubscriptionListAction {
        let action_id = ActionId::new(action_type);
        let mut fund_records: Vec<SubscriptionRecord> = funds
            .iter()
            .map(|fund| {
                return SubscriptionRecord::new(fund, &action_id);
            })
            .collect();
        fund_records.sort_by_key(|fund_record| {
            return fund_record.get_company_name().clone();
        });
        let pager = PagerAction::new(action_id.clone(), fund_records.len());
        let mut action = SubscriptionListAction {
            action_id,
            pager,
            fund_records,
            outgoing_message_id: OutgoingMessageId::new(),
        };
        action.update_subscriptions(subscriptions);
        return action;
    }

    pub fn has_subscriptions(&self) -> bool {
        return self
            .iter_all()
            .nth(0)
            .is_some();
    }

    pub fn new_fund_list(funds: &[Fund], subscriptions: &[FundId]) -> SubscriptionListAction {
        return SubscriptionListAction::new(ActionType::SUBSCRIPTION_LIST, funds, subscriptions);
    }

    pub fn get_outgoing_message_id(&self) -> &OutgoingMessageId {
        return &self.outgoing_message_id;
    }

    pub fn update_subscriptions(&mut self, subscriptions: &[FundId]) {
        for fund_record in self.fund_records.iter_mut() {
            fund_record.is_subscribed = subscriptions.contains(&fund_record.fund_id);
        }
    }

    fn iter_all(&self) -> impl Iterator<Item=&'_ SubscriptionRecord> + '_ {
        let iterator = self
            .fund_records
            .iter()
            .filter(|fund_record| {
                return fund_record.is_subscribed();
            });
        return iterator;
    }

    pub fn iter(&self) -> impl Iterator<Item=&'_ SubscriptionRecord> + '_ {
        let iterator = self.iter_all();
        return self.pager.iter_items(iterator);
    }

    pub fn get_pager(&self) -> &PagerAction {
        return &self.pager;
    }

    pub fn decide(&self, action_route: &ActionRoute) -> SubscriptionListActionDecision {
        for fund_record in self.fund_records.iter() {
            if fund_record.get_route_view() == action_route {
                return SubscriptionListActionDecision::View(fund_record.get_fund_id().clone());
            }
            if fund_record.get_route_unsubscribe() == action_route {
                return SubscriptionListActionDecision::Unsubscribe(fund_record.get_fund_id().clone());
            }
        }
        if let Some(page) = self.pager.get_page_by_route(action_route) {
            return SubscriptionListActionDecision::SelectPage(page.clone());
        }
        return SubscriptionListActionDecision::UnknownRoute;
    }

    pub fn select_page(&mut self, page: &Page) -> Result<(), Failure> {
        return self.pager.select_page(page);
    }
}

impl Entity for SubscriptionListAction {
    type Id = ActionId;
    fn get_entity_id(&self) -> &ActionId {
        return &self.action_id;
    }
}