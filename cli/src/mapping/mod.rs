mod untrack;

use clap::Subcommand;
use interfaces::models::Settings;
use untrack::Untrack;

#[derive(Subcommand, Debug)]
pub enum Mapping {
    Untrack(Untrack)
}

impl Mapping {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		match self {
			Mapping::Untrack(untrack) => untrack.execute(settings).await,
		}
	}
}