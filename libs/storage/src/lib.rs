mod migrations;
mod datasets;

use crate::migrations::migrate;
use interfaces::models::Datasets;
use std::path::PathBuf;

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
				todo!()
			}
		}
	}

	pub fn store(&self, data: &Datasets) -> anyhow::Result<()> {
		match self {
			Storage::Local(path) => {
				datasets::store(&path.join(DB_NAME), data)
			}
			Storage::Remote(_) => {
				todo!()
			}
		}
	}

	pub fn read(&self, name: String, version: u32) -> anyhow::Result<Datasets> {
		match self {
			Storage::Local(path) => {
				datasets::read(&path.join(DB_NAME), name, version)
			}
			Storage::Remote(_) => {
				todo!()
			}
		}
	}
}