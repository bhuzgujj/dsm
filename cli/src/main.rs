mod configuration;
mod format;
mod generator;
mod list;
mod merge;
mod store;
mod mapping;

use clap::Parser;
use interfaces::logger;
use mapping::Mapping;
use storage::Storage;
use std::fs::create_dir_all;

use interfaces::models::{DsmSets, Settings};

use crate::configuration::Configuration;
use crate::generator::Generator;
use crate::list::List;
use crate::merge::Merge;
use crate::store::Store;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
enum Cli {
    Store(Store),
    Gen(Generator),
    Ls(List),
    
    #[command(subcommand)]
    Map(Mapping),

    #[command(subcommand)]
    Merge(Merge),

    Config(Configuration),
    Try,
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
        Cli::Ls(ls) => ls.execute(&settings).await?,
        Cli::Merge(merge) => merge.execute(&settings).await?,
        Cli::Map(map) => map.execute(&settings).await?,
        Cli::Try => {
            let remote = settings.get_remotes().get("my_account").unwrap();
            Storage::Remote { service: remote.clone() }
                .read::<DsmSets>("".to_string(), "1".to_string()).await?;
        }
    };
    settings.save()?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match wrapper().await {
        Ok(()) => Ok(()),
        Err(_) => Ok(()),
    }
}
