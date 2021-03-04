use typed_di::sync_context::sync_container_declaration::SyncContainerDeclaration;
use typed_di::service_id_resolver::ServiceIdResolver;
use typed_di::argument_id_resolver::ArgumentIdResolver;
use typed_di::service_id::ServiceId;
use typed_di::container_declaration::ContainerDeclaration;
use typed_di::error::BuildError;
use crate::sec_gov::model::edgar_file::EdgarFile;
use crate::sec_gov::service::edgar_api::EdgarApi;
use crate::sec_gov::repository::edgar_cache::EdgarCache;
use crate::repository::file_storage::storage_instance::StorageInstance;
use crate::system::app_config::AppConfig;
use crate::system::di;

pub const EDGAR_API: ServiceId<EdgarApi> = EdgarApi::SERVICE_ID;
pub const EDGAR_CACHE: ServiceId<EdgarCache> = EdgarCache::SERVICE_ID;
pub const EDGAR_FILE_STORAGE: ServiceId<StorageInstance<EdgarFile>> = ServiceIdResolver::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), BuildError> {
    builder.register(EDGAR_API, |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_edgar_api();
        let service = EdgarApi::new(
            config,
            resolver.get_service(di::fetching_di::HTTP_CLIENT)?,
            resolver.get_service(EDGAR_CACHE)?,
        );
        return Ok(service);
    })?;
    builder.register(EDGAR_CACHE, |resolver| {
        let service = EdgarCache::new(
            resolver.get_service(EDGAR_FILE_STORAGE)?,
        );
        return Ok(service);
    })?;
    // builder.register(EDGAR_FILE_STORAGE, |resolver| {

    // })?;
    return Ok(());
}