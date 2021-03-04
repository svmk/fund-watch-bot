use typed_di::sync_context::sync_container_declaration::SyncContainerDeclaration;
use typed_di::service_id_resolver::ServiceIdResolver;
use typed_di::argument_id_resolver::ArgumentIdResolver;
use typed_di::service_id::ServiceId;
use typed_di::container_declaration::ContainerDeclaration;
use typed_di::error::BuildError;
use crate::system::di;
use crate::system::app_config::AppConfig;
use crate::openfigi::service::openfigi_api::OpenFigiApi;
use crate::market::common::model::cusip::Cusip;
use crate::openfigi::model::cusip_cache_record::CusipCacheRecord;
use crate::openfigi::path_resolver::cusip_cache_path_mapper::cusip_cache_path_mapper;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::repository::repository::file_repository::FileRepository;
use crate::serializer::service::json_serializer::JsonSerializer;

pub const CUSIP_CACHE: ServiceId<RepositoryInstance<Cusip, CusipCacheRecord>> = ServiceIdResolver::SERVICE_ID;
pub const OPENFIGI_API: ServiceId<OpenFigiApi> = OpenFigiApi::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), BuildError> {
    builder.register(OPENFIGI_API, |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_openfigi_api();
        let service = OpenFigiApi::new(
            config,
            resolver.get_service(di::fetching_di::HTTP_CLIENT)?,
            resolver.get_service(CUSIP_CACHE)?,
        );
        return Ok(service);
    })?;
    builder.register(CUSIP_CACHE, |resolver|{
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            cusip_cache_path_mapper(path),
            JsonSerializer::new(),
            resolver.get_service(di::repository_di::QUERY_COMPARATOR)?,
        );
        return Ok(service);
    })?;
    return Ok(());
}