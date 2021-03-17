use typed_di::argument::argument_id_resolver::ArgumentIdResolver;
use typed_di::async_di::container_declaration::ContainerDeclaration;
use typed_di::async_di::container::Container;
use std::path::Path;
use crate::prelude::*;
use crate::system::di;
use crate::system::app_config::AppConfig;
use crate::system::app_config_loader::app_config_from_path;

pub fn create_di_contaner(path: &Path) -> Result<Container, Failure> {
    let config = app_config_from_path(path)?;
    let mut builder = ContainerDeclaration::new();
    builder.add_argument(AppConfig::ARGUMENT_ID, config)?;
    di::console_di::register_services(&mut builder)?;
    di::event_emitter_di::register_services(&mut builder)?;
    di::fetching_di::register_services(&mut builder)?;
    di::market_fund_report_di::register_services(&mut builder)?;
    di::sec_gov_di::register_services(&mut builder)?;
    di::openfigi_di::register_services(&mut builder)?;
    di::market_data_di::register_services(&mut builder)?;
    di::yahoo_finance_di::register_services(&mut builder)?;
    di::telegram_di::register_services(&mut builder)?;
    di::repository_di::register_services(&mut builder)?;
    let container = builder.build();
    return Ok(container);
}

pub async fn configure_services(container: &Container) -> Result<(), Failure> {
    di::market_fund_report_di::configure_services(container).await?;
    return Ok(());
}