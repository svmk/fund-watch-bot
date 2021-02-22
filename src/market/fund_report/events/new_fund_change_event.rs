use crate::market::fund_report::model::fund_changes_id::FundChangesId;
use crate::event_emitter::model::event::Event;

#[derive(new, Debug, Clone)]
pub struct NewFundChangeEvent {
    fund_change_id: FundChangesId,
}

impl NewFundChangeEvent {
    pub fn get_fund_change_id(&self) -> &FundChangesId {
        return &self.fund_change_id;
    }
}

impl Event for NewFundChangeEvent {}