use crate::prelude::*;
use typed_di::service::service::Service;
use crate::sec_gov::model::company_report_ref::CompanyReportRef;
use crate::sec_gov::model::processed_reports::ProcessedReports;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::app::model::date::Date;

#[derive(new)]
pub struct ReportProcessingCache {
    processed_reports_repository: Service<RepositoryInstance<Date, ProcessedReports>>,
}

impl ReportProcessingCache {
    pub async fn need_fetch(&self, company_report_ref: &CompanyReportRef) -> Result<bool, Failure> {
        let processed_reports = self
            .processed_reports_repository
            .find(company_report_ref.get_date()).await?;
        let processed_reports = match processed_reports {
            Some(processed_reports) => {processed_reports},
            None => {
                ProcessedReports::new(company_report_ref.get_date().clone())
            },
        };
        let was_processed = processed_reports.was_processed(company_report_ref);
        return Ok(!was_processed);
    }

    pub async fn notify_processed(&self, company_report_ref: &CompanyReportRef) -> Result<(), Failure> {
        let processed_reports = self
            .processed_reports_repository
            .find(company_report_ref.get_date()).await?;
        let mut processed_reports = match processed_reports {
            Some(processed_reports) => {processed_reports},
            None => {
                ProcessedReports::new(company_report_ref.get_date().clone())
            },
        };
        processed_reports.add_report_ref(company_report_ref.clone());
        self.processed_reports_repository.store(&processed_reports).await?;
        return Ok(());
    }
}