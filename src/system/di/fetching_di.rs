use typed_di::sync_context::sync_container_declaration::SyncContainerDeclaration;
use typed_di::service_id_resolver::ServiceIdResolver;
use typed_di::argument_id_resolver::ArgumentIdResolver;
use typed_di::container_declaration::ContainerDeclaration;
use typed_di::error::BuildError;
use crate::system::app_config::AppConfig;
use crate::fetching::service::http_client::HttpClient;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), BuildError> {
    builder.register(HttpClient::SERVICE_ID, |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_http_client();
        let service = HttpClient::new(config)?;
        return Ok(service);
    })?;
    return Ok(());
}