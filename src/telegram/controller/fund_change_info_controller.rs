use crate::telegram::{controller::prelude::*, model::action_id::ActionId};
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::market::fund_report::model::fund_changes_id::FundChangesId;
use crate::market::fund_report::model::fund_changes::FundChanges;
use crate::market::market_data::service::candlestick_provider::CandlestickProvider;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::telegram::views::fund_change_info_view::fund_change_info_view;
use crate::telegram::action::fund_change_info_action::FundChangeInfoAction;
use crate::telegram::action::fund_change_record::FundChangeRecord;
use typed_di::service::service::Service;

#[derive(new)]
pub struct FundChangeInfoController {
    fund_repository: Service<RepositoryInstance<Fund>>,
    fund_changes_repository: Service<RepositoryInstance<FundChanges>>,
    action_repository: Service<RepositoryInstance<FundChangeInfoAction>>,
    candlestick_provider: Service<CandlestickProvider>,
}

impl FundChangeInfoController {
    pub async fn render(&self, fund_changes_id: &FundChangesId) -> Result<View, Failure> {
        let fund_id = fund_changes_id.get_prev_fund_id().get_fund_id();
        let fund = self.fund_repository.get(fund_id).await?;
        let fund_change = self
            .fund_changes_repository
            .get(fund_changes_id).await?;
        let mut action = FundChangeInfoAction::new(
            &fund,
        );
        let datetime = fund_changes_id.get_next_fund_id().get_date().end_of_day();
        for buy in fund_change.generate_buys() {
            let split_rules = self
                .candlestick_provider
                .fetch_split_rules(buy.get_company_id(), &datetime).await?;
            let buy = FundChangeRecord::from_buy(&buy, &split_rules)?;
            action.push_buy(buy);
        }
        for sell in fund_change.generate_sells() {
            let split_rules = self
                .candlestick_provider
                .fetch_split_rules(sell.get_company_id(), &datetime).await?;
            let sell = FundChangeRecord::from_sell(&sell, &split_rules)?;
            action.push_sell(sell);
        }
        self.action_repository.store(&action).await?;
        let view = fund_change_info_view(&action);
        return Ok(view);
    }
}