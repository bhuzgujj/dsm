use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DsmLocation {
    Remote { url: String },
    Local { path: String },
}

impl DsmLocation {
    pub fn image(&self, rel_path: &PathBuf) -> PathBuf {
        match self {
            DsmLocation::Remote { url: _ } => todo!("Remote image not implemented"),
            DsmLocation::Local { path } => PathBuf::from(path).join(rel_path),
        }
    }
}
