mod configuration;
mod format;
mod generator;
mod list;
mod mapping;
mod merge;
mod store;
mod serve;

use clap::Parser;
use colored::Colorize;
use log::info;
use mapping::Mapping;
use std::fs::create_dir_all;

use crate::configuration::Configuration;
use crate::generator::Generator;
use crate::list::List;
use crate::merge::Merge;
use crate::store::Store;
use interfaces::models::{requires_migration, Settings, LEDGER_CURRENT_VERSION};
use interfaces::paths::log_path;
use crate::serve::Serve;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
enum Cli {
	Store(Store),
	Generate(Generator),

	#[command(subcommand)]
	List(List),

	#[command(subcommand)]
	Map(Mapping),

	#[command(subcommand)]
	Merge(Merge),

	Serve(Serve),

	Config(Configuration),
}

async fn execute(settings: &mut Settings) -> anyhow::Result<()> {
	bhomz::logger::bind_logger(settings.log_level(), Some(log_path()))?;
	create_dir_all(settings.store_path())?;
	create_dir_all(settings.ledger_path())?;
	if requires_migration(settings.ledger_version()) {
		info!("Migrate to version {LEDGER_CURRENT_VERSION}");
		storage::migrate(settings).await?;
		settings.update_ledger_version();
		settings.save()?;
	}

	match Cli::parse() {
		Cli::Store(store) => store.execute(settings).await,
		Cli::Generate(generator) => generator.execute(settings).await,
		Cli::List(ls) => ls.execute(settings).await,
		Cli::Merge(merge) => merge.execute(settings).await,
		Cli::Map(map) => map.execute(settings).await,
		Cli::Serve(server) => server.start(settings),
		Cli::Config(configuration) => configuration.configure(settings),
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut settings = Settings::load();
	if let Err(err) = execute(&mut settings).await {
		println!("{}", err.to_string().red());
	}
	settings.save()
}
