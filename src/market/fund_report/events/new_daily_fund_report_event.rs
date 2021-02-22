use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::event_emitter::model::event::Event;

#[derive(new, Debug, Clone)]
pub struct NewDailyFundReportEvent {
    daily_fund_report_id: DailyFundReportId,
}

impl NewDailyFundReportEvent {
    pub fn get_daily_fund_report_id(&self) -> &DailyFundReportId {
        return &self.daily_fund_report_id;
    }
}

impl Event for NewDailyFundReportEvent {}