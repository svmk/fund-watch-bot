
use typed_di::service::service_id_resolver::ServiceIdResolver;
use typed_di::argument::argument_id_resolver::ArgumentIdResolver;
use typed_di::service::service_id::ServiceId;
use typed_di::async_di::container_declaration::ContainerDeclaration;
use typed_di::error::Error;

use crate::sec_gov::model::edgar_file::EdgarFile;
use crate::sec_gov::model::processed_reports::ProcessedReports;
use crate::sec_gov::service::edgar_api::EdgarApi;
use crate::sec_gov::repository::report_processing_cache::ReportProcessingCache;
use crate::sec_gov::repository::edgar_cache::EdgarCache;
use crate::sec_gov::path_resolver::edgar_cache_path_resolver::edgar_cache_path_resolver;
use crate::sec_gov::path_resolver::report_processing_path_resolver::report_processing_path_resolver;
use crate::repository::file_storage::storage_instance::StorageInstance;
use crate::repository::file_storage::file_storage::FileStorage;
use crate::repository::repository::file_repository::FileRepository;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::serializer::service::json_serializer::JsonSerializer;
use crate::system::app_config::AppConfig;
use crate::system::di;

pub const EDGAR_API: ServiceId<EdgarApi> = EdgarApi::SERVICE_ID;
pub const EDGAR_CACHE: ServiceId<EdgarCache> = EdgarCache::SERVICE_ID;
pub const EDGAR_FILE_STORAGE: ServiceId<StorageInstance<EdgarFile>> = ServiceIdResolver::SERVICE_ID;
pub const REPORT_PROCESSING_REPOSITORY: ServiceId<RepositoryInstance<ProcessedReports>> = ServiceIdResolver::SERVICE_ID;
pub const REPORT_PROCESSING_CACHE: ServiceId<ReportProcessingCache> = ServiceIdResolver::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), Error> {
    builder.register(EDGAR_API, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_edgar_api();
        let service = EdgarApi::new(
            config,
            resolver.get_service(di::fetching_di::HTTP_CLIENT).await?,
            resolver.get_service(EDGAR_CACHE).await?,
        );
        return Ok(service);
    })?;
    builder.register(EDGAR_CACHE, async move |resolver| {
        let service = EdgarCache::new(
            resolver.get_service(EDGAR_FILE_STORAGE).await?,
            resolver.get_service(di::event_emitter_di::EVENT_EMITTER).await?,
        );
        return Ok(service);
    })?;
    builder.register(EDGAR_FILE_STORAGE, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let path = config.get_repository().get_path();
        let service = FileStorage::new(
            edgar_cache_path_resolver(path),
        );
        return Ok(service);
    })?;
    builder.register(REPORT_PROCESSING_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let path = config.get_repository().get_path();
        let service = FileRepository::new(
            report_processing_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    builder.register(REPORT_PROCESSING_CACHE, async move |resolver| {
        let service = ReportProcessingCache::new(
            resolver.get_service(REPORT_PROCESSING_REPOSITORY).await?,
        );
        return Ok(service);
    })?;
    return Ok(());
}