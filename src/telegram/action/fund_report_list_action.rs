use crate::prelude::*;
use crate::telegram::model::action_id::ActionId;
use crate::market::common::model::company_name::CompanyName;
use crate::telegram::model::action_type::ActionType;
use crate::telegram::model::action_route::ActionRoute;
use crate::telegram::action::pager_action::{PagerAction, Page};
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::repository::model::entity::Entity;
use crate::telegram::model::outgoing_message_id::OutgoingMessageId;

#[derive(Debug, Serialize, Deserialize)]
pub struct FundReportRecord {
    #[serde(rename="fund_report_id")]
    fund_report_id: DailyFundReportId,
    #[serde(rename="route_view")]
    route_view: ActionRoute,
}

impl FundReportRecord {
    fn new(fund_report_id: DailyFundReportId, action_id: &ActionId) -> FundReportRecord {
        return FundReportRecord {
            fund_report_id,
            route_view: action_id.create_route(),
        }
    }
    pub fn get_fund_report_id(&self) -> &DailyFundReportId {
        return &self.fund_report_id;
    }

    pub fn get_route_view(&self) -> &ActionRoute {
        return &self.route_view;
    }
}


#[derive(Debug)]
pub enum FundReportListActionDecision {
    View(DailyFundReportId),
    SelectPage(Page),
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundReportListAction {
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
    #[serde(rename="fund_report_records")]
    fund_report_records: Vec<FundReportRecord>,
}

impl FundReportListAction {
    pub fn new(fund: &Fund, fund_reports: &[DailyFundReportId]) -> FundReportListAction {
        let action_id = ActionId::new(ActionType::FUND_REPORT_LIST);
        let mut fund_report_records: Vec<_> = fund_reports
            .iter()
            .map(|fund_report| {
                return FundReportRecord::new(fund_report.clone(), &action_id);
            })
            .collect();
        fund_report_records.sort_by(|a, b| {
            let a = a.get_fund_report_id().get_date();
            let b = b.get_fund_report_id().get_date();
            return b.cmp(a);
        });
        return FundReportListAction {
            action_id: action_id.clone(),
            fund_id: fund.get_fund_id().clone(),
            company_name: fund.get_company_name().clone(),
            outgoing_message_id: OutgoingMessageId::new(),
            pager: PagerAction::new(action_id, fund_report_records.len()),
            fund_report_records,
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

    pub fn iter(&self) -> impl Iterator<Item=&'_ FundReportRecord> + '_ {
        let iterator = self
            .fund_report_records
            .iter();
        return self.pager.iter_items(iterator);
    }

    pub fn decide(&self, action_route: &ActionRoute) -> FundReportListActionDecision {
        for fund_report_record in self.fund_report_records.iter() {
            if fund_report_record.get_route_view() == action_route {
                return FundReportListActionDecision::View(fund_report_record.get_fund_report_id().clone());
            }
        }
        if let Some(page) = self.pager.get_page_by_route(action_route) {
            return FundReportListActionDecision::SelectPage(page.clone());
        }
        return FundReportListActionDecision::Unknown;
    }

    pub fn select_page(&mut self, page: &Page) -> Result<(), Failure> {
        return self.pager.select_page(page);
    }
}

impl Entity for FundReportListAction {
    type Id = ActionId;
    fn get_entity_id(&self) -> &ActionId {
        return &self.action_id;
    }
}