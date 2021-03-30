use crate::telegram::model::action_id::ActionId;
use crate::telegram::model::action_type::ActionType;
use crate::telegram::model::outgoing_message_id::OutgoingMessageId;
use crate::market::fund_report::model::fund_component_buy::FundComponentBuy;
use crate::market::fund_report::model::fund_component_sell::FundComponentSell;
use crate::market::fund_report::model::fund_changes::FundChanges;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::market::common::model::company_name::CompanyName;
use crate::repository::model::entity::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct FundChangeInfoAction {
    #[serde(rename="outgoing_message_id")]
    outgoing_message_id: OutgoingMessageId,
    #[serde(rename="action_id")]
    action_id: ActionId,
    #[serde(rename = "buys")]
    buys: Vec<FundComponentBuy>,
    #[serde(rename = "sells")]
    sells: Vec<FundComponentSell>,
    #[serde(rename="fund_id")]
    fund_id: FundId,
    #[serde(rename="fund_name")]
    fund_name: CompanyName,
}

impl FundChangeInfoAction {
    pub fn new(fund: &Fund, fund_changes: &FundChanges) -> FundChangeInfoAction {
        let action_id = ActionId::new(ActionType::FUND_CHANGE_INFO);
        let buys = fund_changes.generate_buys();
        let sells = fund_changes.generate_sells();
        return FundChangeInfoAction {
            outgoing_message_id: OutgoingMessageId::new(),
            action_id: action_id.clone(),
            buys,
            sells,
            fund_id: fund.get_fund_id().clone(),
            fund_name: fund.get_company_name().clone(),
        }
    }

    pub fn get_outgoing_message_id(&self) -> &OutgoingMessageId {
        return &self.outgoing_message_id;
    }

    pub fn get_fund_id(&self) -> &FundId {
        return &self.fund_id;
    }
    
    pub fn get_fund_name(&self) -> &CompanyName {
        return &self.fund_name;
    }

    pub fn get_buys(&self) -> &Vec<FundComponentBuy> {
        return &self.buys;
    }
    
    pub fn get_sells(&self) -> &Vec<FundComponentSell> {
        return &self.sells;
    }
}

impl Entity<ActionId> for FundChangeInfoAction {
    fn get_entity_id(&self) -> &ActionId {
        return &self.action_id;
    }
}