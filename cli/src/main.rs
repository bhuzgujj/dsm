mod store;
mod format;
mod configuration;
mod generator;
mod list;
mod merge;

use std::fs::{create_dir_all};
use clap::Parser;
use log::error;

use interfaces::logger;
use interfaces::models::{Settings};

use crate::configuration::Configuration;
use crate::generator::Generator;
use crate::list::List;
use crate::merge::Merge;
use crate::store::Store;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
enum Cli {
    #[command(subcommand)]
    Store(Store),

    #[command(subcommand)]
    Gen(Generator),

    #[command(subcommand)]
    List(List),

    #[command(subcommand)]
    Merge(Merge),

    Config(Configuration)
}

async fn wrapper() -> anyhow::Result<()> {
    let mut settings = Settings::load();
    logger::bind_logger(&settings)?;
    create_dir_all(settings.get_ledger_path())?;
    create_dir_all(settings.get_store_path())?;
    match Cli::parse() {
        Cli::Store(store) => store.execute(&settings).await?,
        Cli::Gen(generator) => generator.execute(&settings).await?,
        Cli::Config(configuration) => configuration.configure(&mut settings)?,
        Cli::List(ls) => ls.execute(&settings).await?,
        Cli::Merge(merge) => merge.execute(&settings).await?
    };
    settings.save()?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match wrapper().await {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("Error: {}", e);
            Ok(())
        }
    }
}