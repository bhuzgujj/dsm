mod configuration;
mod format;
mod generator;
mod list;
mod mapping;
mod merge;
mod migration;
mod store;

use clap::Parser;
use interfaces::logger;
use mapping::Mapping;
use std::fs::create_dir_all;

use interfaces::models::{requires_migration, Settings};

use crate::configuration::Configuration;
use crate::generator::Generator;
use crate::list::List;
use crate::merge::Merge;
use crate::store::Store;

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

	Config(Configuration),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut settings = Settings::load();
	logger::bind_logger(&settings)?;
	create_dir_all(settings.store_path())?;
	create_dir_all(settings.ledger_path())?;
	if requires_migration(settings.ledger_version()) {
		migration::migrate(&settings).await?;
	}

	match Cli::parse() {
		Cli::Store(store) => store.execute(&settings).await?,
		Cli::Generate(generator) => generator.execute(&settings).await?,
		Cli::List(ls) => ls.execute(&settings).await?,
		Cli::Merge(merge) => merge.execute(&settings).await?,
		Cli::Map(map) => map.execute(&settings).await?,

		Cli::Config(configuration) => configuration.configure(&mut settings)?,
	};
	settings.save()
}
