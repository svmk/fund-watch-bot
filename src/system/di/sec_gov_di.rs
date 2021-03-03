use typed_di::sync_context::sync_container_declaration::SyncContainerDeclaration;
use typed_di::service_id_resolver::ServiceIdResolver;
use typed_di::argument_id_resolver::ArgumentIdResolver;
use typed_di::service_id::ServiceId;
use typed_di::container_declaration::ContainerDeclaration;
use typed_di::error::BuildError;
use crate::sec_gov::service::edgar_api::EdgarApi;

pub const EDGAR_API: ServiceId<EdgarApi> = EdgarApi::SERVICE_ID;
pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), BuildError> {
    return Ok(());
}