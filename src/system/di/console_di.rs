
use typed_di::service::service_id_resolver::ServiceIdResolver;
use typed_di::async_di::container_declaration::ContainerDeclaration;
use typed_di::service::service_id::ServiceId;
use typed_di::error::Error;
use crate::system::di;
use crate::console::service::import_13f_form_service::Import13Form;
use crate::console::service::run_telegram_service::RunTelegram;

pub const IMPORT_13F_FORM_SERVICE: ServiceId<Import13Form> = ServiceIdResolver::SERVICE_ID;
pub const RUN_TELEGRAM_SERVICE: ServiceId<RunTelegram> = ServiceIdResolver::SERVICE_ID;
pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), Error> {
    builder.register(IMPORT_13F_FORM_SERVICE, async move |resolver| {
        let service = Import13Form::new(
            resolver.get_service(di::market_fund_report_di::DAILY_FUND_REPORT_IMPORTING).await?,
            resolver.get_service(di::event_emitter_di::EVENT_LISTENER).await?,
        );
        return Ok(service);
    })?;
    builder.register(RUN_TELEGRAM_SERVICE, async move |resolver| {
        let service = RunTelegram::new(
            resolver.get_service(di::telegram_di::TELEGRAM_BOT_TASK).await?,
        );
        return Ok(service);
    })?;
    return Ok(());
}