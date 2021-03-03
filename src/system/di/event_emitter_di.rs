use typed_di::sync_context::sync_container_declaration::SyncContainerDeclaration;
use typed_di::service_id_resolver::ServiceIdResolver;
use typed_di::container_declaration::ContainerDeclaration;
use typed_di::service_id::ServiceId;
use typed_di::error::BuildError;
use crate::event_emitter::service::event_emitter::EventEmitter;
use crate::event_emitter::service::event_processing::EventProcessing;

pub const EVENT_EMITTER: ServiceId<EventEmitter> = ServiceIdResolver::SERVICE_ID;
pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), BuildError> {
    builder.register(EventProcessing::SERVICE_ID, |resolver| {
        let service = EventProcessing::new();
        return Ok(service);
    })?;
    builder.register(EventEmitter::SERVICE_ID, |resolver| {
        let service = EventEmitter::new(
            resolver.get_service(EventProcessing::SERVICE_ID)?,
        );
        return Ok(service);
    })?;
    return Ok(());
}