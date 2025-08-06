use std::collections::HashMap;

use crate::models::{ClassMapper, LicenseMapper};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StorableMerged {
    name: String,
    version: String,
    datasets: HashMap<String, GroupSet>,
    class_mapper: ClassMapper,
    license_mapper: LicenseMapper,
}

impl StorableMerged {
    pub fn new(
        name: String,
        version: String,
        datasets: HashMap<String, GroupSet>,
        class_mapper: ClassMapper,
        license_mapper: LicenseMapper,
    ) -> Self {
        Self {
            name,
            version,
            datasets,
            class_mapper,
            license_mapper,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn version(&self) -> &String {
        &self.version
    }

    pub fn datasets(&self) -> &HashMap<String, GroupSet> {
        &self.datasets
    }

    pub fn class_mapper(&self) -> &ClassMapper {
        &self.class_mapper
    }

    pub fn license_mapper(&self) -> &LicenseMapper {
        &self.license_mapper
    }
}

#[derive(Serialize, Deserialize)]
pub struct GroupSet {
    group: String,
    location: Location,
}

impl GroupSet {
    pub fn local(group: String) -> Self {
        Self {
            group,
            location: Location::Local,
        }
    }

    pub fn group(&self) -> &String {
        &self.group
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Serialize, Deserialize)]
pub enum Location {
    Remote { url: String },
    Local,
}
