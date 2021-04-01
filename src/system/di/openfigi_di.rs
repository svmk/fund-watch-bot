
use typed_di::service::service_id_resolver::ServiceIdResolver;
use typed_di::argument::argument_id_resolver::ArgumentIdResolver;
use typed_di::service::service_id::ServiceId;
use typed_di::async_di::container_declaration::ContainerDeclaration;
use typed_di::error::Error;
use crate::system::di;
use crate::system::app_config::AppConfig;
use crate::openfigi::service::openfigi_api::OpenFigiApi;

use crate::openfigi::model::cusip_cache_record::CusipCacheRecord;
use crate::openfigi::path_resolver::cusip_cache_path_resolver::cusip_cache_path_resolver;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::repository::repository::file_repository::FileRepository;
use crate::serializer::service::json_serializer::JsonSerializer;

pub const CUSIP_CACHE: ServiceId<RepositoryInstance<CusipCacheRecord>> = ServiceIdResolver::SERVICE_ID;
pub const OPENFIGI_API: ServiceId<OpenFigiApi> = OpenFigiApi::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), Error> {
    builder.register(OPENFIGI_API, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_openfigi_api();
        let service = OpenFigiApi::new(
            config,
            resolver.get_service(di::fetching_di::HTTP_CLIENT).await?,
            resolver.get_service(CUSIP_CACHE).await?,
        );
        return Ok(service);
    })?;
    builder.register(CUSIP_CACHE, async move |resolver|{
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            cusip_cache_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    return Ok(());
}