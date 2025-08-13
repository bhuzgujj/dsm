use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Remote {
	Azure {
		storage_container: String,
		storage_account: String,
	},
}

impl Display for Remote {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Remote::Azure {
				storage_container,
				storage_account,
			} => {
				f.write_str(format!("Azure(storage_container: '{storage_container}', storage_account: '{storage_account}')").as_str())
			},
		}
	}
}
