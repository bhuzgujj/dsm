mod migrations;
mod datasets;

use interfaces::models::Datasets;
use std::path::PathBuf;
use anyhow::Error;
use crate::migrations::migrate;

const DB_NAME: &'static str = "datasets.sqlite";

pub enum Storage {
	Local(PathBuf),
	Remote(String)
}

impl Storage {
	pub fn initialize(&self) -> anyhow::Result<()> {
		match self {
			Storage::Local(path) => {
				migrate(&path.join(DB_NAME))
			}
			Storage::Remote(_) => {
				Err(Error::msg("Remote Not implemented"))
			}
		}
	}

	pub fn store(&self, data: &Datasets) -> anyhow::Result<()> {
		match self {
			Storage::Local(path) => {
				datasets::store(&path.join(DB_NAME), data)
			}
			Storage::Remote(_) => {
				Err(Error::msg("Remote Not implemented"))
			}
		}
	}
}