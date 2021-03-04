use typed_di::sync_context::sync_container_declaration::SyncContainerDeclaration;
use typed_di::service_id_resolver::ServiceIdResolver;
use typed_di::container_declaration::ContainerDeclaration;
use typed_di::service_id::ServiceId;
use typed_di::error::BuildError;
use crate::system::di;
use crate::console::service::import_13f_form_service::Import13Form;

pub const IMPORT_13F_FORM_SERVICE: ServiceId<Import13Form> = ServiceIdResolver::SERVICE_ID;
pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), BuildError> {
    builder.register(IMPORT_13F_FORM_SERVICE, |resolver| {
        let service = Import13Form::new(
            resolver.get_service(di::market_fund_report_di::DAILY_FUND_REPORT_IMPORTING)?,
        );
        return Ok(service);
    })?;
    return Ok(());
}