use typed_di::sync_context::sync_container_declaration::SyncContainerDeclaration;
use typed_di::service_id_resolver::ServiceIdResolver;
use typed_di::argument_id_resolver::ArgumentIdResolver;
use typed_di::service_id::ServiceId;
use typed_di::container_declaration::ContainerDeclaration;
use typed_di::error::BuildError;
use crate::system::di;
use crate::system::app_config::AppConfig;
use crate::yahoo_finance::service::yahoo_api::YahooApi;

pub const YAHOO_API: ServiceId<YahooApi> = ServiceIdResolver::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), BuildError> {
    builder.register(YAHOO_API, |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_yahoo_api();
        let service = YahooApi::new(
            config,
            resolver.get_service(di::fetching_di::HTTP_CLIENT,)?,
        );
        return Ok(service);
    })?;
    return Ok(());
}