use crate::app::model::year_quartal::YearQuartal;
use crate::market::common::model::share::Share;
use crate::market::fund_report::model::daily_fund_report::DailyFundReport;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::market::fund_report::model::fund::Fund;
use crate::market::fund_report::model::fund_component::FundComponent;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::weight::Weight;
use crate::market::market_data::service::candlestick_provider::CandlestickProvider;
use crate::openfigi::service::openfigi_api::OpenFigiApi;
use crate::prelude::*;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::sec_gov::model::company_report_ref::CompanyReportRef;
use crate::sec_gov::service::edgar_api::EdgarApi;
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
    report_repository: Service<RepositoryInstance<DailyFundReportId, DailyFundReport>>,
    candlestick_provider: Service<CandlestickProvider>,
}

impl DailyFundReportImporting {
    pub async fn import_reports(
        &self,
        quartal: &YearQuartal,
    ) -> Result<Vec<DailyFundReportRef>, Failure> {
        let edgar_company_index = self.edgar_api.fetch_company_index(quartal).await?;
        let mut report_refs: Vec<_> = edgar_company_index
            .iter()
            .filter(|report_ref| {
                return report_ref.get_form_type().is_13f();
            })
            .map(|report_ref| {
                return DailyFundReportRef::new(report_ref.clone());
            })
            .collect();
        report_refs.sort_by_key(|report_ref| {
            return report_ref.get_company_report_ref().get_date().clone();
        });
        return Ok(report_refs);
    }

    pub async fn fetch_daily_fund_report(
        &self,
        report_ref: &DailyFundReportRef,
    ) -> Result<DailyFundReport, Failure> {
        let report_ref = report_ref.get_company_report_ref();
        let report = self.edgar_api.fetch_compoany_report_13f(report_ref).await?;
        let fund_id = FundId::from_cik(report_ref.get_cik().clone())?;
        let fund = match self.fund_repository.find(&fund_id).await? {
            Some(fund) => fund,
            None => Fund::new(fund_id, report.get_form_13f().get_company_name().clone()),
        };
        let daily_fund_report_id = DailyFundReportId::new(
            fund.get_fund_id().clone(),
            report_ref.get_date().clone(),
        );
        if let Some(result) = self.report_repository.find(&daily_fund_report_id).await? {
            return Ok(result);
        }
        let mut result = DailyFundReport::new(daily_fund_report_id);
        let share_sum = report
            .get_information_table()
            .iter_components()
            .map(|component| {
                return component.get_share().clone();
            })
            .fold(Share::zero(), |accumulator, share| {
                return accumulator.add(share);
            });
        let share_sum = share_sum.into_f64();
        let report_datetime = report.get_form_13f().get_period_of_report().end_of_day();
        for fund_component in report.get_information_table().iter_components() {
            let ticker = self
                .openfigi_api
                .get_ticker_by_cusip(fund_component.get_cusip())
                .await?;
            let weight = fund_component.get_share().clone().into_f64() / share_sum;
            let weight = Weight::from_f64(weight)?;
            let candlestick = self
                .candlestick_provider
                .fetch_historical_candlestick(ticker.clone(), report_datetime.clone())
                .await?;
            let fund_component = FundComponent::new(
                ticker.clone(),
                fund_component.get_share().clone(),
                candlestick.get_daily().get_close().clone(),
                weight,
            );
            result.add_fund_component(fund_component);
        }
        self.report_repository.store(&result).await?;
        return Ok(result);
    }
}
