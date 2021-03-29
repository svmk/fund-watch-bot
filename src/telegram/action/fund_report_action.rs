use std::cmp::Ordering;

use crate::prelude::*;
use crate::telegram::model::action_id::ActionId;
use crate::app::model::date::Date;
use crate::market::common::model::company_name::CompanyName;
use crate::telegram::model::action_type::ActionType;
use crate::telegram::model::action_route::ActionRoute;
use crate::telegram::action::pager_action::{PagerAction, Page};
use crate::market::common::model::ticker::Ticker;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::common::model::actual_price::ActualPrice;
use crate::market::common::model::actual_volume::ActualVolume;
use crate::market::fund_report::model::fund::Fund;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::market::fund_report::model::weight::Weight;
use crate::market::market_data::model::split_rules::SplitRules;
use crate::market::fund_report::model::daily_fund_report::DailyFundReport;
use crate::market::fund_report::model::fund_component::FundComponent;
use crate::repository::model::entity::Entity;
use crate::telegram::model::outgoing_message_id::OutgoingMessageId;

#[derive(Debug, Serialize, Deserialize)]
pub struct FundComponentRecord {
    #[serde(rename="ticker")]
    ticker: Ticker,
    #[serde(rename="price")]
    price: ActualPrice,
    #[serde(rename="volume")]
    volume: ActualVolume,
    #[serde(rename="weight")]
    weight: Weight,
}

impl FundComponentRecord {
    pub fn new(component: &FundComponent, split_rules: &SplitRules) -> Result<FundComponentRecord, Failure> {
        let price = split_rules
            .calculate_actual_price(component.get_share().get_price())?;
        let volume = split_rules
            .calculate_actual_volume(component.get_share().get_share())?;
        return Ok(FundComponentRecord {
            ticker: component.get_ticker().clone(),
            price,
            volume,
            weight: component.get_share().get_weight().clone(),
        });
    }

    pub fn get_ticker(&self) -> &Ticker {
        return &self.ticker;
    }

    pub fn get_price(&self) -> &ActualPrice {
        return &self.price;
    }

    pub fn get_volume(&self) -> &ActualVolume {
        return &self.volume;
    }

    pub fn get_weight(&self) -> &Weight {
        return &self.weight;
    }
}


#[derive(Debug)]
pub enum FundReportActionDecision {
    SelectPage(Page),
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundReportAction {
    #[serde(rename="fund_name")]
    fund_name: CompanyName,
    #[serde(rename="report_date")]
    report_date: Date,
    #[serde(rename="action_id")]
    action_id: ActionId,
    #[serde(rename="outgoing_message_id")]
    outgoing_message_id: OutgoingMessageId,
    #[serde(rename="pager")]
    pager: PagerAction,
    #[serde(rename="fund_component_records")]
    fund_component_records: Vec<FundComponentRecord>,
}

impl FundReportAction {
    pub fn new_empty(fund: &Fund, fund_report: &DailyFundReport) -> FundReportAction {
        let action_id = ActionId::new(ActionType::FUND_REPORT);
        return FundReportAction {
            action_id: action_id.clone(),
            fund_name: fund.get_company_name().clone(),
            report_date: fund_report.get_id().get_date().clone(),
            outgoing_message_id: OutgoingMessageId::new(),
            pager: PagerAction::new(action_id, fund_report.get_fund_components().len()),
            fund_component_records: Vec::with_capacity(fund_report.get_fund_components().len()),
        }
    }

    pub fn push_component(&mut self, component: &FundComponent, split_rules: &SplitRules) -> Result<(), Failure> {
        let fund_component_record = FundComponentRecord::new(
            component,
            split_rules,
        )?;
        self.fund_component_records.push(fund_component_record);
        self.fund_component_records.sort_by(|a, b| {
            let a = a.get_weight();
            let b = b.get_weight();
            return b.partial_cmp(a).unwrap_or(Ordering::Equal);
        });
        return Ok(());
    }

    pub fn get_fund_name(&self) -> &CompanyName {
        return &self.fund_name;
    }

    pub fn get_report_date(&self) -> &Date {
        return &self.report_date;
    }

    pub fn get_outgoing_message_id(&self) -> &OutgoingMessageId {
        return &self.outgoing_message_id;
    }

    pub fn get_pager(&self) -> &PagerAction {
        return &self.pager;
    }

    pub fn iter(&self) -> impl Iterator<Item=&'_ FundComponentRecord> + '_ {
        let iterator = self
            .fund_component_records
            .iter();
        return self.pager.iter_items(iterator);
    }

    pub fn decide(&self, action_route: &ActionRoute) -> FundReportActionDecision {
        if let Some(page) = self.pager.get_page_by_route(action_route) {
            return FundReportActionDecision::SelectPage(page.clone());
        }
        return FundReportActionDecision::Unknown;
    }

    pub fn select_page(&mut self, page: &Page) -> Result<(), Failure> {
        return self.pager.select_page(page);
    }
}

impl Entity<ActionId> for FundReportAction {
    fn get_entity_id(&self) -> &ActionId {
        return &self.action_id;
    }
}