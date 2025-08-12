use clap::Subcommand;
use interfaces::models::Settings;

use crate::list::{actions::Actions, datasets::Datasets};

mod actions;
mod datasets;

/// List accessable data
#[derive(Subcommand, Debug)]
pub enum List {
	Datasets(Datasets),
	Actions(Actions),
}

impl List {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		match self {
			List::Datasets(datasets) => datasets.execute(settings).await,
			List::Actions(actions) => actions.execute(settings).await,
		}
	}
}
