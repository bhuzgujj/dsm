mod std;
mod custom;

use clap::Subcommand;
use interfaces::models::Settings;
use crate::generator::custom::Custom;
use crate::generator::std::Std;

/// Add a files to the files store
#[derive(Subcommand, Debug)]
pub enum Generator {
	Custom(Custom),
	Std(Std)
}

impl Generator {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		match &self {
			Generator::Custom(fmt) => fmt.execute(settings).await,
			Generator::Std(fmt) => fmt.execute(settings).await,
		}
	}
}