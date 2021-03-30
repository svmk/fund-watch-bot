use crate::app::model::year_quartal::YearQuartal;
use crate::app::model::year_quartal_iterator::YearQuartalIterator;
use crate::market::common::model::share::Share;
use crate::market::fund_report::model::daily_fund_report::DailyFundReport;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::market::fund_report::model::fund::Fund;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund_component::FundComponent;
use crate::market::fund_report::model::weight::Weight;
use crate::market::fund_report::model::daily_fund_report_import_request::DailyFundReportImportRequest;
use crate::market::fund_report::events::new_daily_fund_report_event::NewDailyFundReportEvent;
use crate::market::market_data::service::candlestick_provider::CandlestickProvider;
use crate::event_emitter::service::event_emitter::EventEmitter;
use crate::openfigi::service::openfigi_api::OpenFigiApi;
use crate::prelude::*;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::sec_gov::model::company_report_ref::CompanyReportRef;
use crate::sec_gov::service::edgar_api::EdgarApi;
use crate::sec_gov::repository::report_processing_cache::ReportProcessingCache;
use typed_di::service::service::Service;

#[derive(new)]
pub struct DailyFundReportImporting {
    edgar_api: Service<EdgarApi>,
    openfigi_api: Service<OpenFigiApi>,
    fund_repository: Service<RepositoryInstance<FundId, Fund>>,
    report_repository: Service<RepositoryInstance<DailyFundReportId, DailyFundReport>>,
    candlestick_provider: Service<CandlestickProvider>,
    event_emitter: Service<EventEmitter>,
    report_processing_cache: Service<ReportProcessingCache>,
}

impl DailyFundReportImporting {
    pub async fn import_period(&self, request: DailyFundReportImportRequest) -> Result<(), Failure> {
        let started_at = request.get_started_at().clone();
        let ended_at = request.get_ended_at().clone();
        let start_quartal = YearQuartal::from_date(started_at.clone());
        let end_quartal = match ended_at {
            Some(ref ended_at) => {
                YearQuartal::from_date(ended_at.clone())
            },
            None => {
                YearQuartal::now()
            },
        };
        let iterator = YearQuartalIterator::new(start_quartal, end_quartal)?;
        for year_quartal in iterator {
            let fund_report_refs = self.import_reports(&year_quartal).await?;
            let fund_report_refs_iterator = fund_report_refs
                .iter()
                .filter(|fund_report_ref| {
                    let datetime = fund_report_ref.get_date();
                    if datetime < &started_at {
                        return false;
                    }
                    if let Some(ended_at) = ended_at.as_ref() {
                        if &datetime > &ended_at {
                            return false;
                        }
                    }
                    return true;
                });
            for fund_report_ref in fund_report_refs_iterator {
                let mut need_fetch = true;
                if request.get_process_only_new() {
                    need_fetch = self
                        .report_processing_cache
                        .need_fetch(fund_report_ref).await?;
                }
                if need_fetch {
                    let _ = self.fetch_daily_fund_report(fund_report_ref).await?;
                }
            }
        }
        return Ok(());
    }

    pub async fn import_reports(
        &self,
        quartal: &YearQuartal,
    ) -> Result<Vec<CompanyReportRef>, Failure> {
        let edgar_company_index = self.edgar_api.fetch_company_index(quartal).await?;
        let mut report_refs: Vec<_> = edgar_company_index
            .iter()
            .filter(|report_ref| {
                let form_type = report_ref.get_form_type();
                if form_type.is_13f() {
                    return true;
                }
                return false;
            })
            .map(Clone::clone)
            .collect();
        report_refs.sort_by_key(|report_ref| {
            return report_ref.get_date().clone();
        });
        return Ok(report_refs);
    }

    pub async fn fetch_daily_fund_report(
        &self,
        report_ref: &CompanyReportRef,
    ) -> Result<Option<DailyFundReport>, Failure> {
        let fund_report = self.inner_fetch_daily_fund_report(report_ref).await?;
        self.report_processing_cache.notify_processed(report_ref).await?;
        return Ok(fund_report);
    }

    async fn inner_fetch_daily_fund_report(
        &self,
        report_ref: &CompanyReportRef,
    ) -> Result<Option<DailyFundReport>, Failure> {
        let report = self.edgar_api.fetch_compoany_report_13f(report_ref).await?;
        let report = match report {
            Some(report) => report,
            None => {
                return Ok(None);
            },
        };
        let fund_id = FundId::from_cik(report_ref.get_cik().clone())?;
        let fund = match self.fund_repository.find(&fund_id).await? {
            Some(fund) => fund,
            None => Fund::new(fund_id, report.get_form_13f().get_company_name().clone()),
        };
        self.fund_repository.store(&fund).await?;
        let daily_fund_report_id = DailyFundReportId::new(
            fund.get_fund_id().clone(),
            report_ref.get_date().clone(),
        );
        if let Some(result) = self.report_repository.find(&daily_fund_report_id).await? {
            return Ok(Some(result));
        }
        let mut result = DailyFundReport::new(daily_fund_report_id);
        let share_sum = report
            .get_information_table()
            .iter_components()
            .map(|component| {
                return component.get_share().clone();
            })
            .fold(Share::zero(), |accumulator, share| {
                return accumulator.sum(&share);
            });
        let share_sum = share_sum.into_f64();
        let report_datetime = report.get_form_13f().get_period_of_report().end_of_day();
        for fund_component in report.get_information_table().iter_components() {
            let ticker = self
                .openfigi_api
                .get_ticker_by_cusip(fund_component.get_cusip())
                .await?;
            // TODO: Компонент фонда, для которого не был найден тикер, всё равно должен попасть в фонд.
            let ticker = match ticker {
                Some(ticker) => ticker,
                None => {
                    continue;
                }
            };
            let weight = fund_component.get_share().clone().into_f64() / share_sum;
            let weight = Weight::from_f64(weight)?;
            let candlestick_result = self
                .candlestick_provider
                .fetch_last_candlestick(ticker.clone(), report_datetime.clone())
                .await;
            let candlestick = match candlestick_result {
                Ok(candlestick) => Some(candlestick),
                Err(error) => {
                    if error.is_ticker_not_available() {
                        None
                    } else {
                        return Err(error.into());
                    }
                },
            };
            if let Some(candlestick) = candlestick {
                // TODO: Изменить модель FundComponent, и сделать цены опциональными
                let fund_component = FundComponent::new(
                    ticker.clone(),
                    fund_component.get_share().clone(),
                    candlestick.get_orignal().get_close().clone(),
                    weight,
                );
                result.add_fund_component(fund_component);
            }
        }
        self.report_repository.store(&result).await?;
        self.event_emitter.emit_event(NewDailyFundReportEvent::new(result.get_id().clone())).await?;
        return Ok(Some(result));
    }
}
