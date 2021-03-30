use crate::market::fund_report::model::daily_fund_report::DailyFundReport;
use crate::market::fund_report::model::fund_component::FundComponent;
use crate::market::common::error::fund_changes_error::FundChangesError;
use crate::market::fund_report::model::fund_component_change::FundComponentChange;
use crate::market::fund_report::model::fund_component_buy::FundComponentBuy;
use crate::market::fund_report::model::fund_component_sell::FundComponentSell;
use crate::market::fund_report::model::share_change::ShareChange;
use crate::market::fund_report::model::price_change::PriceChange;
use crate::market::fund_report::model::weight_change::WeightChange;
use crate::market::fund_report::model::fund_changes_id::FundChangesId;
use crate::repository::model::entity::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct FundChanges {
    #[serde(rename= "id")]
    id: FundChangesId,
    #[serde(rename= "added_to_fund")]
    added_to_fund: Vec<FundComponent>,
    #[serde(rename= "removed_from_fund")]
    removed_from_fund: Vec<FundComponent>,
    #[serde(rename= "fund_component_changes")]
    fund_component_changes: Vec<FundComponentChange>,
}

impl FundChanges {
    pub fn generate(prev_report: &DailyFundReport, next_report: &DailyFundReport) -> Result<FundChanges, FundChangesError> {
        if prev_report.get_id().get_fund_id() != next_report.get_id().get_fund_id() {
            return Err(FundChangesError::FundIdDiffer);
        }
        if prev_report.get_id() == next_report.get_id() {
            return Err(FundChangesError::SameDailyReports);
        }
        let mut prev_fund_components = prev_report.get_fund_components().to_vec();
        prev_fund_components.sort_by(|a, b| {
            return a.get_ticker().cmp(b.get_ticker());
        });
        let mut next_fund_components = next_report.get_fund_components().to_vec();
        next_fund_components.sort_by(|a, b| {
            return a.get_ticker().cmp(b.get_ticker());
        });
        let id = FundChangesId::new(
            prev_report.get_id().clone(),
            next_report.get_id().clone(),
        );
        let mut result = FundChanges {
            id,
            added_to_fund: Vec::new(),
            removed_from_fund: Vec::new(),
            fund_component_changes: Vec::new(),
        };
        for next_fund_component in next_fund_components.iter() {
            let prev_fund = prev_fund_components
                .binary_search_by(|item| {
                    return item.get_ticker().cmp(next_fund_component.get_ticker());
                })
                .ok()
                .and_then(|prev_fund_component_index| {
                    return prev_fund_components.get(prev_fund_component_index);
                });
            match prev_fund {
                Some(prev_fund_component) => {
                    result.update_fund_component(prev_fund_component, next_fund_component);
                },
                None => {
                    result.push_added_fund_component(next_fund_component.clone());
                },
            }
        }
        for prev_fund_component in prev_fund_components.iter() {
            let is_next_fund_component_found = next_fund_components.binary_search_by(|item| {
                return item.get_ticker().cmp(prev_fund_component.get_ticker());
            })
            .is_ok();
            if !is_next_fund_component_found {
                result.push_removed_fund_component(prev_fund_component.clone());
            }
        }
        return Ok(result);
    }

    fn push_added_fund_component(&mut self, component: FundComponent) {
        self.added_to_fund.push(component);
    }

    fn update_fund_component(&mut self, old_component: &FundComponent, new_component: &FundComponent) {
        let share_change = ShareChange::new(
            old_component.get_share().get_share().clone(), 
            new_component.get_share().get_share().clone(),
        );
        let price_change = PriceChange::new(
            old_component.get_share().get_price().clone(), 
            new_component.get_share().get_price().clone(),
        );
        let weight_change = WeightChange::new(
            old_component.get_share().get_weight().clone(), 
            new_component.get_share().get_weight().clone(),
        );
        let fund_component_change = FundComponentChange::new(
            old_component.get_ticker().clone(),
            share_change,
            price_change,
            weight_change,
        );
        self.fund_component_changes.push(fund_component_change);
    }

    fn push_removed_fund_component(&mut self, component: FundComponent) {
        self.removed_from_fund.push(component);
    }

    pub fn get_id(&self) -> &FundChangesId {
        return &self.id;
    }

    pub fn generate_buys(&self) -> Vec<FundComponentBuy> {
        let mut result = Vec::new();
        for added_component in self.added_to_fund.iter() {
            let added_component = FundComponentBuy::new(
                added_component.get_ticker().clone(),
                added_component.get_share().get_share().clone(),
                added_component.get_share().get_price().clone(),
                added_component.get_share().get_weight().clone(),
            );
            result.push(added_component);
        }
        for fund_component_change in self.fund_component_changes.iter() {
            if let Some(buy_component) = fund_component_change.generate_fund_component_buy() {
                result.push(buy_component);
            }
        }
        return result;
    }

    pub fn generate_sells(&self) -> Vec<FundComponentSell> {
        let mut result = Vec::new();
        for removed_component in self.removed_from_fund.iter() {
            let removed_component = FundComponentSell::new(
                removed_component.get_ticker().clone(),
                removed_component.get_share().get_share().clone(),
                removed_component.get_share().get_price().clone(),
                removed_component.get_share().get_weight().clone(),
            );
            result.push(removed_component);
        }
        for fund_component_change in self.fund_component_changes.iter() {
            if let Some(sell_component) = fund_component_change.generate_fund_component_sell() {
                result.push(sell_component);
            }
        }
        return result;
    }
}

impl Entity<FundChangesId> for FundChanges {
    fn get_entity_id(&self) -> &FundChangesId {
        return &self.id;
    }
}