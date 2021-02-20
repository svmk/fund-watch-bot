use crate::market::fund_report::model::daily_fund_report::DailyFundReport;
use crate::market::fund_report::model::fund_component::FundComponent;
use crate::market::common::error::fund_changes_error::FundChangesError;
use crate::market::fund_report::model::fund_component_change::FundComponentChange;
use crate::market::fund_report::model::share_change::ShareChange;
use crate::market::fund_report::model::price_change::PriceChange;
use crate::market::fund_report::model::weight_change::WeightChange;
use crate::market::fund_report::model::fund_changes_id::FundChangesId;

#[derive(Debug)]
pub struct FundChanges {
    id: FundChangesId,
    added_to_fund: Vec<FundComponent>,
    removed_from_fund: Vec<FundComponent>,
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
        let mut fund_component_change = FundComponentChange::new(old_component.get_ticker().clone());
        let share_change = ShareChange::new(
            old_component.get_share().get_share().clone(), 
            new_component.get_share().get_share().clone(),
        );
        if let Some(share_change) = share_change {
            fund_component_change.set_share_change(share_change);
        }
        let price_change = PriceChange::new(
            old_component.get_share().get_price().clone(), 
            new_component.get_share().get_price().clone(),
        );
        if let Some(price_change) = price_change {
            fund_component_change.set_price_change(price_change);
        }
        let weight_change = WeightChange::new(
            old_component.get_share().get_weight().clone(), 
            new_component.get_share().get_weight().clone(),
        );
        if let Some(weight_change) = weight_change {
            fund_component_change.set_weight_change(weight_change);
        }
        self.fund_component_changes.push(fund_component_change);
    }

    fn push_removed_fund_component(&mut self, component: FundComponent) {
        self.removed_from_fund.push(component);
    }
}