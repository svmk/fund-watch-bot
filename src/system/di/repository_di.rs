use typed_di::sync_context::sync_container_declaration::SyncContainerDeclaration;
use typed_di::service_id_resolver::ServiceIdResolver;
use typed_di::service_id::ServiceId;
use typed_di::container_declaration::ContainerDeclaration;
use typed_di::error::BuildError;
use crate::repository::service::query_comparator::QueryComparator;

pub const QUERY_COMPARATOR: ServiceId<QueryComparator> = ServiceIdResolver::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), BuildError> {
    builder.register(QUERY_COMPARATOR, |resolver| {
        let service = QueryComparator::new();
        return Ok(service);
    })?;
    return Ok(());
}