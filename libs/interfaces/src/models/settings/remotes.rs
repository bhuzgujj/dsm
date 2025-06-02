use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Remote {
    Azure {
        storage_container: String,
        storage_account: String,
    },
}

