#![feature(arbitrary_self_types)]
#![feature(raw)]
#![feature(const_fn)]
#![feature(hash_drain_filter)]
#![feature(async_closure)]
#![feature(drain_filter)]

extern crate tokio;
#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_value_object;
#[macro_use]
extern crate async_trait;

mod app;
mod fetching;
mod prelude;
mod error;
mod market;
mod repository;
mod serializer;
mod sec_gov;
mod openfigi;
mod yahoo_finance;
mod telegram;
mod event_emitter;
mod system;
mod console;
use crate::console::console_application::ConsoleApplication;
use crate::system::app_config::AppConfig;
use crate::system::di::application_di::{create_di_contaner, configure_services};
use crate::system::console_execution::execute_console;
use crate::system::app_config_loader::app_config_from_path;
use crate::error::failure::Failure;
use std::mem::forget;
use structopt::StructOpt;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let console_argument = ConsoleApplication::from_args();
    let config = app_config_from_path(&console_argument.config_path)?;
    if let Some(sentry_dsn) = config.get_opt_sentry() {
        let sentry_guard = sentry::init(sentry_dsn.as_str());
        forget(sentry_guard);
    }
    let result = run(console_argument, config).await;
    if let Err(error) = result {
        sentry::integrations::anyhow::capture_anyhow(&error);
        return Err(error.into());
    }
    return Ok(());
}


async fn run(console_argument: ConsoleApplication, config: AppConfig) -> Result<(), Failure> {
    let container = create_di_contaner(config)
        .map_err(|error| {
            return crate::error!("Unable to create di container: {}", error);
        })?;
    configure_services(&container).await?;
    execute_console(container, &console_argument).await?;
    return Ok(());
}