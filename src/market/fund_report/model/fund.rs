use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;

#[derive(Debug)]
pub struct Fund {
    fund_id: FundId,
    last_fund_report_id: Option<DailyFundReportId>,
}

impl Fund {
    pub fn new(fund_id: FundId) -> Fund {
        return Fund {
            fund_id,
            last_fund_report_id: None,
        };
    }

    pub fn update_last_fund_report_id(&mut self, last_fund_report_id: DailyFundReportId) {
        self.last_fund_report_id = Some(last_fund_report_id);
    }
}