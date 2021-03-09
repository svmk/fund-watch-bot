
use typed_di::service::service_id_resolver::ServiceIdResolver;
use typed_di::service::service_id::ServiceId;
use typed_di::argument::argument_id_resolver::ArgumentIdResolver;
use typed_di::async_di::container_declaration::ContainerDeclaration;
use typed_di::error::Error;
use crate::system::di;
use crate::system::app_config::AppConfig;
use crate::market::fund_report::service::daily_fund_report_importing::DailyFundReportImporting;
use crate::market::fund_report::service::fund_changes_generator::FundChangesGenerator;
use crate::market::fund_report::service::fund_reports_event_listener::FundReportsEventListener;
use crate::market::fund_report::model::daily_fund_report::DailyFundReport;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::market::fund_report::model::fund_changes::FundChanges;
use crate::market::fund_report::model::fund_reports::FundReports;
use crate::market::fund_report::model::fund_changes_id::FundChangesId;
use crate::market::fund_report::path_resolver::fund_reports_path_resolver::fund_reports_path_resolver;
use crate::market::fund_report::path_resolver::fund_path_resolver::fund_path_resolver;
use crate::market::fund_report::path_resolver::daily_fund_report_path_resolver::daily_fund_report_path_resolver;
use crate::serializer::service::json_serializer::JsonSerializer;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::repository::repository::file_repository::FileRepository;
use crate::market::market_data::service::candlestick_provider::CandlestickProvider;

pub const FUND_REPOSITORY: ServiceId<RepositoryInstance<FundId, Fund>> = ServiceIdResolver::SERVICE_ID;
pub const DAILY_FUND_REPORT_REPOSITORY: ServiceId<RepositoryInstance<DailyFundReportId, DailyFundReport>> = ServiceIdResolver::SERVICE_ID;
pub const CANDLESTICK_PROVIDER: ServiceId<CandlestickProvider> = ServiceIdResolver::SERVICE_ID;
pub const DAILY_FUND_REPORT_IMPORTING: ServiceId<DailyFundReportImporting> = ServiceIdResolver::SERVICE_ID;
pub const FUND_CHANGES_GENERATOR: ServiceId<FundChangesGenerator> = ServiceIdResolver::SERVICE_ID;
pub const FUND_CHANGES_REPOSITORY: ServiceId<RepositoryInstance<FundChangesId, FundChanges>> = ServiceIdResolver::SERVICE_ID;
pub const FUND_REPORTS_EVENT_LISTENER: ServiceId<FundReportsEventListener> = ServiceIdResolver::SERVICE_ID;
pub const FUND_REPORTS_REPOSITORY: ServiceId<RepositoryInstance<FundId, FundReports>> = ServiceIdResolver::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), Error> {
    builder.register(DAILY_FUND_REPORT_IMPORTING, async move |resolver| {
        let service = DailyFundReportImporting::new(
            resolver.get_service(di::sec_gov_di::EDGAR_API).await?,
            resolver.get_service(di::openfigi_di::OPENFIGI_API).await?,
            resolver.get_service(FUND_REPOSITORY).await?,
            resolver.get_service(DAILY_FUND_REPORT_REPOSITORY).await?,
            resolver.get_service(CANDLESTICK_PROVIDER).await?,
            resolver.get_service(di::event_emitter_di::EVENT_EMITTER).await?,
        );
        return Ok(service);
    })?;
    builder.register(FUND_CHANGES_GENERATOR, async move |resolver| {
        let service = FundChangesGenerator::new(
            resolver.get_service(DAILY_FUND_REPORT_REPOSITORY).await?,
            resolver.get_service(FUND_CHANGES_REPOSITORY).await?,
            resolver.get_service(di::event_emitter_di::EVENT_EMITTER).await?,
        );
        return Ok(service);
    })?;
    builder.register(FUND_REPORTS_EVENT_LISTENER, async move |resolver| {
        let service = FundReportsEventListener::new(
            resolver.get_service(FUND_CHANGES_GENERATOR).await?,
            resolver.get_service(FUND_REPORTS_REPOSITORY).await?,
        );
        return Ok(service);
    })?;
    builder.register(FUND_REPORTS_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            fund_reports_path_resolver(path),
            JsonSerializer::new(),
            resolver.get_service(di::repository_di::QUERY_COMPARATOR).await?,
        );
        return Ok(service);
    })?;
    builder.register(FUND_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            fund_path_resolver(path),
            JsonSerializer::new(),
            resolver.get_service(di::repository_di::QUERY_COMPARATOR).await?,
        );
        return Ok(service);
    })?;
    builder.register(DAILY_FUND_REPORT_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            daily_fund_report_path_resolver(path),
            JsonSerializer::new(),
            resolver.get_service(di::repository_di::QUERY_COMPARATOR).await?,
        );
        return Ok(service);
    })?;
    return Ok(());
}