use crate::prelude::*;
use crate::market::fund_report::service::daily_fund_report_importing::DailyFundReportImporting;
use crate::app::model::year_quartal::YearQuartal;
use crate::app::model::datetime::DateTime;
use crate::event_emitter::service::event_listener::EventListener;
use crate::event_emitter::model::event_record::EventRecord;
use crate::console::event_handlers::WATCH_EDGAR_CACHE_ACCESS;
use crate::sec_gov::events::edgar_cache_access_event::EdgarCacheAccessEvent;
use typed_di::service::service::Service;

#[derive(new)]
pub struct Import13Form {
    daily_fund_report_importing: Service<DailyFundReportImporting>,
    event_listener: Service<EventListener>,
}

impl Import13Form {
    pub async fn run(&self) -> Result<(), Failure> {
        let _event1 = self
            .event_listener
            .listen(WATCH_EDGAR_CACHE_ACCESS)
            .within_sender_context(async move |event: EventRecord<EdgarCacheAccessEvent>| {
                println!("Processing `{}`", event.get_payload().get_url());
                return Ok(());
            }).await?;
        println!("Started!");
        let start_at = YearQuartal::from_datetime(DateTime::ymd_start_day(1993, 1, 1));
        let end_at = YearQuartal::now();
        self.daily_fund_report_importing.import_period(start_at, end_at).await?;
        return Ok(());
    }
}