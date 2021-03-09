
use typed_di::service::service_id_resolver::ServiceIdResolver;
use typed_di::service::service_id::ServiceId;
use typed_di::async_di::container_declaration::ContainerDeclaration;
use typed_di::error::Error;
use crate::repository::service::query_comparator::QueryComparator;

pub const QUERY_COMPARATOR: ServiceId<QueryComparator> = ServiceIdResolver::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), Error> {
    builder.register_ready(QUERY_COMPARATOR, async move |resolver| {
        return QueryComparator::new();
    })?;
    return Ok(());
}