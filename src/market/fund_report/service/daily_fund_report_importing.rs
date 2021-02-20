use crate::sec_gov::service::edgar_api::EdgarApi;
use crate::sec_gov::model::company_report_ref::CompanyReportRef;
use crate::openfigi::service::openfigi_api::OpenFigiApi;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::app::model::year_quartal::YearQuartal;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::daily_fund_report::DailyFundReport;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::market::fund_report::model::fund_component::FundComponent;
use crate::market::fund_report::model::fund_component_share::FundComponentShare;
use crate::market::fund_report::model::fund::Fund;
use crate::prelude::*;
use typed_di::service::Service;

#[derive(new, Debug)]
pub struct DailyFundReportRef {
    company_report_ref: CompanyReportRef,
}

impl DailyFundReportRef {
    pub fn get_company_report_ref(&self) -> &CompanyReportRef {
        return &self.company_report_ref;
    }
}

#[derive(new)]
pub struct DailyFundReportImporting {
    edgar_api: Service<EdgarApi>,
    openfigi_api: Service<OpenFigiApi>,
    fund_repository: Service<RepositoryInstance<FundId, Fund>>,
}

impl DailyFundReportImporting {
    pub async fn import_reports(&self, quartal: &YearQuartal) -> Result<Vec<DailyFundReportRef>, Failure> {
        let edgar_company_index = self
            .edgar_api
            .fetch_company_index(quartal).await?;
        let report_refs: Vec<_> = edgar_company_index
            .iter()
            .filter(|report| {
                return report.get_form_type().is_13f();
            })
            .map(|report | {
                return DailyFundReportRef::new(report.clone());
            })
            .collect();
        return Ok(report_refs);
    }

    async fn fetch_report(&self, report_ref: &DailyFundReportRef) -> Result<Option<DailyFundReport>, Failure> {
        let report_ref= report_ref.get_company_report_ref();
        let report = self
            .edgar_api
            .fetch_compoany_report_13f(report_ref).await?;
        let fund_id = FundId::from_cik(report_ref.get_cik().clone())?;
        let fund = match self.fund_repository.find(&fund_id).await? {
            Some(fund) => fund,
            None => {
                Fund::new(fund_id, report.get_form_13f().get_company_name().clone())
            },
        };
        let mut result = DailyFundReport::new(fund.get_fund_id().clone());
        for fund_component in report.get_information_table().iter_components() {
            let ticker = self
                .openfigi_api
                .get_ticker_by_cusip(fund_component.get_cusip()).await?;
            // let fund_component_share = FundComponentShare::new(

            // );
        }
        unimplemented!()
    }
}