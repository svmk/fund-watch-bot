
use typed_di::service::service_id_resolver::ServiceIdResolver;
use typed_di::service::service_id::ServiceId;
use typed_di::argument::argument_id_resolver::ArgumentIdResolver;
use typed_di::async_di::container_declaration::ContainerDeclaration;
use typed_di::error::Error;
use crate::system::app_config::AppConfig;
use crate::system::di;
use crate::fetching::service::http_client::HttpClient;

pub const HTTP_CLIENT: ServiceId<HttpClient> = ServiceIdResolver::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), Error> {
    builder.register(HTTP_CLIENT, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_http_client();
        let service = HttpClient::new(
            config,
            resolver.get_service(di::event_emitter_di::EVENT_EMITTER).await?,
        )?;
        return Ok(service);
    })?;
    return Ok(());
}