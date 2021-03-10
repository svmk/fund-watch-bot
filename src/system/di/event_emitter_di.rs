
use typed_di::service::service_id_resolver::ServiceIdResolver;
use typed_di::async_di::container_declaration::ContainerDeclaration;
use typed_di::service::service_id::ServiceId;
use typed_di::error::Error;
use crate::event_emitter::service::event_emitter::EventEmitter;
use crate::event_emitter::service::event_listener::EventListener;

pub const EVENT_EMITTER: ServiceId<EventEmitter> = ServiceIdResolver::SERVICE_ID;
pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), Error> {
    builder.register(EventListener::SERVICE_ID, async move |resolver| {
        let service = EventListener::new();
        return Ok(service);
    })?;
    builder.register(EventEmitter::SERVICE_ID, async move |resolver| {
        let service = EventEmitter::new(
            resolver.get_service(EventListener::SERVICE_ID).await?,
        );
        return Ok(service);
    })?;
    return Ok(());
}