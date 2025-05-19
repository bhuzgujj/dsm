use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::dataform::DataForm;
use crate::models::datasets::classes::Classes;
use crate::models::datasets::licence::License;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetaData {
    name: String,
    version: u32,
    subset_version: Option<String>,
    contributor: String,
    date_created: String,
    description: String,
    url: String,
    year: String,
    formatter: DataForm,
    classes: HashMap<u32, Classes>,
    licenses: HashMap<u32, License>,
}

impl MetaData {
    pub fn new(
        name: String,
        version: u32,
        subset_version: Option<String>,
        contributor: String,
        date_created: String,
        description: String,
        url: String,
        year: String,
        formatter: DataForm,
        classes: HashMap<u32, Classes>,
        licenses: HashMap<u32, License>
    ) -> Self {
        Self {
            name,
            version,
            subset_version,
            contributor,
            date_created,
            description,
            url,
            year,
            formatter,
            classes,
            licenses,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_version(&self) -> u32 {
        self.version.clone()
    }
    
    pub fn get_classes(&self) -> HashMap<u32, Classes> {
        self.classes.clone()
    }
    
    pub fn get_subset_version(&self) -> Option<String> {
        self.subset_version.clone()
    }

    pub fn get_contributor(&self) -> String {
        self.contributor.clone()
    }

    pub fn get_date_created(&self) -> String {
        self.date_created.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_year(&self) -> String {
        self.year.clone()
    }

    pub fn get_formatter(&self) -> DataForm {
        self.formatter.clone()
    }
}