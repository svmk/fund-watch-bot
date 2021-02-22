use crate::market::fund_report::service::daily_fund_report_importing::DailyFundReportImporting;
use crate::market::fund_report::model::fund_id::FundId;
use crate::repository::repository::repository_instance::RepositoryInstance;
use typed_di::service::Service;

pub struct FundChangesGenerator {
    daily_fund_report_importing: Service<DailyFundReportImporting>,
}

impl FundChangesGenerator {
    
}