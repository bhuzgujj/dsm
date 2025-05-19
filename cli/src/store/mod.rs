mod std;
mod custom;

use clap::Subcommand;
use interfaces::models::Settings;
use crate::store::custom::Custom;
pub use crate::store::std::Std;

/// Add a files to the files store
#[derive(Subcommand, Debug)]
pub enum Store {
	Custom(Custom),
	Std(Std)
}

impl Store {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		match &self {
			Store::Custom(fmt) => fmt.execute(settings).await,
			Store::Std(fmt) => fmt.execute(settings).await,
		}
	}
}