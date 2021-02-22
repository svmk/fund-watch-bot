use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::event_emitter::model::event::Event;

#[derive(new, Debug, Clone)]
pub struct NewDailyFundReportEvent {
    daily_fund_report_id: DailyFundReportId,
}

impl Event for NewDailyFundReportEvent {}