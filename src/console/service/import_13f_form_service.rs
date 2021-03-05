use crate::prelude::*;
use crate::market::fund_report::service::daily_fund_report_importing::DailyFundReportImporting;
use crate::app::model::year_quartal::YearQuartal;
use crate::app::model::datetime::DateTime;
use typed_di::service::Service;

#[derive(new)]
pub struct Import13Form {
    daily_fund_report_importing: Service<DailyFundReportImporting>,
}

impl Import13Form {
    pub async fn run(&self) -> Result<(), Failure> {
        let start_at = YearQuartal::from_datetime(DateTime::ymd_start_day(1980, 01, 01));
        let end_at = YearQuartal::now();
        self.daily_fund_report_importing.import_period(start_at, end_at).await?;
        return Ok(());
    }
}