mod dsm;

use crate::mutate::dsm::Dsm;
use clap::{Subcommand};
use interfaces::models::Settings;

/// Mutate metadata
#[derive(Subcommand, Debug)]
pub enum Mutate {
    // Datasets
    Dsm(Dsm)
}

impl Mutate {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		match self {
			Mutate::Dsm(dsm) => dsm.execute(settings).await,
		}
	}
}