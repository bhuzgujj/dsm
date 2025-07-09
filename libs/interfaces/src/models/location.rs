use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DsmLocation {
    Remote {
        url: String
    },
    Local
}