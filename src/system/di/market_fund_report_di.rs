use typed_di::sync_context::sync_container_declaration::SyncContainerDeclaration;
use typed_di::service_id_resolver::ServiceIdResolver;
use typed_di::service_id::ServiceId;
use typed_di::argument_id_resolver::ArgumentIdResolver;
use typed_di::container_declaration::ContainerDeclaration;
use typed_di::error::BuildError;
use crate::system::app_config::AppConfig;
use crate::market::fund_report::service::daily_fund_report_importing::DailyFundReportImporting;
use crate::market::fund_report::service::fund_changes_generator::FundChangesGenerator;
use crate::market::fund_report::service::fund_reports_event_listener::FundReportsEventListener;
use crate::market::fund_report::model::daily_fund_report::DailyFundReport;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::market::market_data::service::candlestick_provider::CandlestickProvider;
use crate::system::di;

pub const FUND_REPOSITORY: ServiceId<RepositoryInstance<FundId, Fund>> = ServiceIdResolver::SERVICE_ID;
pub const REPORT_REPOSITORY: ServiceId<RepositoryInstance<DailyFundReportId, DailyFundReport>> = ServiceIdResolver::SERVICE_ID;
pub const CANDLESTICK_PROVIDER: ServiceId<CandlestickProvider> = ServiceIdResolver::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), BuildError> {
    builder.register(DailyFundReportImporting::SERVICE_ID, |resolver| {
        let service = DailyFundReportImporting::new(
            resolver.get_service(di::sec_gov_di::EDGAR_API)?,
            resolver.get_service(di::openfigi_di::OPENFIGI_API)?,
            resolver.get_service(FUND_REPOSITORY)?,
            resolver.get_service(REPORT_REPOSITORY)?,
            resolver.get_service(CANDLESTICK_PROVIDER)?,
            resolver.get_service(di::event_emitter_di::EVENT_EMITTER)?,
        );
        return Ok(service);
    })?;
    return Ok(());
}