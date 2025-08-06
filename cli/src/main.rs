mod actions;
mod configuration;
mod format;
mod generator;
mod list;
mod mapping;
mod merge;
mod serve;
mod store;

use clap::Parser;
use interfaces::logger;
use mapping::Mapping;
use std::fs::create_dir_all;

use interfaces::models::Settings;

use crate::actions::Actions;
use crate::configuration::Configuration;
use crate::generator::Generator;
use crate::list::List;
use crate::merge::Merge;
use crate::serve::Serve;
use crate::store::Store;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
enum Cli {
	Store(Store),
	Gen(Generator),
	List(List),
	Action(Actions),

	#[command(subcommand)]
	Map(Mapping),

	#[command(subcommand)]
	Merge(Merge),

	Serve(Serve),

	Config(Configuration),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut settings = Settings::load();
	logger::bind_logger(&settings)?;
	create_dir_all(settings.ledger_path())?;
	create_dir_all(settings.store_path())?;
	match Cli::parse() {
		Cli::Action(action) => action.execute(&settings).await?,
		Cli::Store(store) => store.execute(&settings).await?,
		Cli::Gen(generator) => generator.execute(&settings).await?,
		Cli::List(ls) => ls.execute(&settings).await?,
		Cli::Merge(merge) => merge.execute(&settings).await?,
		Cli::Map(map) => map.execute(&settings).await?,

		Cli::Serve(serve) => serve.start(&settings)?,

		Cli::Config(configuration) => configuration.configure(&mut settings)?,
	};
	settings.save()
}
