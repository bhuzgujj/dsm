mod store;
mod format;
mod configuration;

use clap::{Parser, Subcommand};
use log::error;
use interfaces::logger;
use interfaces::models::Settings;
use interfaces::paths::dsm_dir;
use storage::Storage;
use crate::configuration::Configuration;
use crate::store::Store;

#[derive(Parser, Debug)]
enum Cli {
    #[command(subcommand)]
    Store(Store),
    Config(Configuration),
    Test
}

fn wrapper() -> anyhow::Result<()> {
    let mut sets = Settings::load();
    logger::bind_logger(&sets)?;
    Storage::Local(sets.get_dataset_database()).initialize()?;
    match Cli::parse() {
        Cli::Store(store) => store.execute(&sets)?,
        Cli::Config(configuration) => configuration.configure(&mut sets)?,
        Cli::Test => {
        },
    };
    sets.save()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    match wrapper() {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("Error: {}", e);
            Err(e)
        }
    }
}