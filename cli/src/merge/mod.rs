mod new;

use crate::merge::new::New;
use clap::{Subcommand};
use interfaces::models::Settings;

/// Manage merged sets
#[derive(Subcommand, Debug)]
pub enum Merge {
	/// Add new merged set
	New(New),
}

impl Merge {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		match self {
			Merge::New(new) => new.execute(settings).await,
		}
	}
}