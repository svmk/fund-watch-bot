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
use crate::system::di::application_di::create_di_contaner;
use crate::system::console_execution::execute_console;
use structopt::StructOpt;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let console_argument = ConsoleApplication::from_args();
    let container = create_di_contaner(&console_argument.config_path)
        .map_err(|error| {
            return crate::error!("Unable to create di container: {}", error);
        })?;
    execute_console(container, &console_argument).await?;
    return Ok(());
}
