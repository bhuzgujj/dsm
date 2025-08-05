use std::collections::HashMap;

use crate::models::{ClassMapper, LicenseMapper};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StorableMerged {
    pub name: String,
    pub version: String,
    pub datasets: HashMap<String, GroupSet>,
    pub class_mapper: ClassMapper,
    pub license_mapper: LicenseMapper,
}

#[derive(Serialize, Deserialize)]
pub struct GroupSet {
    pub group: String,
    pub location: Location,
}

impl GroupSet {
    pub fn local(group: String) -> Self {
        Self {
            group,
            location: Location::Local,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Location {
    Remote { url: String },
    Local,
}
