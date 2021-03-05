use crate::prelude::*;
use crate::market::fund_report::service::daily_fund_report_importing::DailyFundReportImporting;
use typed_di::service::Service;

#[derive(new)]
pub struct Import13Form {
    daily_fund_report_importing: Service<DailyFundReportImporting>,
}

impl Import13Form {
    pub async fn run(&self) -> Result<(), Failure> {
        // self.daily_fund_report_importing.
        return Ok(());
    }
}