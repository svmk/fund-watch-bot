use crate::prelude::*;
use crate::telegram::model::action_id::ActionId;
use crate::market::common::model::company_name::CompanyName;
use crate::telegram::model::action_type::ActionType;
use crate::telegram::model::action_route::ActionRoute;
use crate::telegram::action::pager_action::{PagerAction, Page};
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::market::fund_report::model::fund_changes_id::FundChangesId;
use crate::repository::model::entity::Entity;
use crate::telegram::model::outgoing_message_id::OutgoingMessageId;

#[derive(Debug, Serialize, Deserialize)]
pub struct FundChangeRecord {
    #[serde(rename="fund_change_id")]
    fund_change_id: FundChangesId,
    #[serde(rename="route_view")]
    route_view: ActionRoute,
}

impl FundChangeRecord {
    fn new(fund_change_id: FundChangesId, action_id: &ActionId) -> FundChangeRecord {
        return FundChangeRecord {
            fund_change_id,
            route_view: action_id.create_route(),
        }
    }

    pub fn get_fund_change_id(&self) -> &FundChangesId {
        return &self.fund_change_id;
    }

    pub fn get_route_view(&self) -> &ActionRoute {
        return &self.route_view;
    }
}


#[derive(Debug)]
pub enum FundChangeListActionDecision {
    View(FundChangesId),
    SelectPage(Page),
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundChangeListAction {
    #[serde(rename="fund_id")]
    fund_id: FundId,
    #[serde(rename="company_name")]
    company_name: CompanyName,
    #[serde(rename="action_id")]
    action_id: ActionId,
    #[serde(rename="outgoing_message_id")]
    outgoing_message_id: OutgoingMessageId,
    #[serde(rename="pager")]
    pager: PagerAction,
    #[serde(rename="fund_changes_records")]
    fund_changes_records: Vec<FundChangeRecord>,
}

impl FundChangeListAction {
    pub fn new(fund: &Fund, fund_changes: &[FundChangesId]) -> FundChangeListAction {
        let action_id = ActionId::new(ActionType::FUND_CHANGE_LIST);
        let mut fund_changes_records: Vec<_> = fund_changes
            .iter()
            .map(|fund_change_id| {
                return FundChangeRecord::new(fund_change_id.clone(), &action_id);
            })
            .collect();
        fund_changes_records.sort_by_key(|fund_change_record| {
            return fund_change_record.get_fund_change_id().get_prev_fund_id().get_date().clone();
        });
        return FundChangeListAction {
            action_id: action_id.clone(),
            fund_id: fund.get_fund_id().clone(),
            company_name: fund.get_company_name().clone(),
            outgoing_message_id: OutgoingMessageId::new(),
            pager: PagerAction::new(action_id, fund_changes_records.len()),
            fund_changes_records,
        }
    }

    pub fn get_company_name(&self) -> &CompanyName {
        return &self.company_name;
    }

    pub fn get_outgoing_message_id(&self) -> &OutgoingMessageId {
        return &self.outgoing_message_id;
    }

    pub fn get_pager(&self) -> &PagerAction {
        return &self.pager;
    }

    pub fn iter(&self) -> impl Iterator<Item=&'_ FundChangeRecord> + '_ {
        let iterator = self
            .fund_changes_records
            .iter();
        return self.pager.iter_items(iterator);
    }

    pub fn decide(&self, action_route: &ActionRoute) -> FundChangeListActionDecision {
        for fund_change_record in self.fund_changes_records.iter() {
            if fund_change_record.get_route_view() == action_route {
                return FundChangeListActionDecision::View(fund_change_record.get_fund_change_id().clone());
            }
        }
        if let Some(page) = self.pager.get_page_by_route(action_route) {
            return FundChangeListActionDecision::SelectPage(page.clone());
        }
        return FundChangeListActionDecision::Unknown;
    }

    pub fn select_page(&mut self, page: &Page) -> Result<(), Failure> {
        return self.pager.select_page(page);
    }
}

impl Entity for FundChangeListAction {
    type Id = ActionId;
    fn get_entity_id(&self) -> &ActionId {
        return &self.action_id;
    }
}