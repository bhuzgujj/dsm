use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Location {
	Remote { url: String },
	Local { path: String },
}

impl Location {
	pub fn image(&self, rel_path: &PathBuf) -> PathBuf {
		match self {
			Location::Remote { url: _ } => todo!("Remote image not implemented"),
			Location::Local { path } => PathBuf::from(path).join(rel_path),
		}
	}
}
