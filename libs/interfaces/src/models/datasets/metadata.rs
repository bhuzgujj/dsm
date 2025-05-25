use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::form::DsmDataForm;
use crate::models::datasets::classes::DsmClasses;
use crate::models::datasets::licence::DsmLicense;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DsmMetaData {
    name: String,
    version: String,
    subset_version: Option<String>,
    contributor: String,
    date_created: String,
    description: String,
    url: String,
    year: String,
    formatter: DsmDataForm,
    classes: HashMap<u32, DsmClasses>,
    licenses: HashMap<u32, DsmLicense>,
}

impl DsmMetaData {
    pub fn new(
        name: String,
        version: String,
        subset_version: Option<String>,
        contributor: String,
        date_created: String,
        description: String,
        url: String,
        year: String,
        formatter: DsmDataForm,
        classes: HashMap<u32, DsmClasses>,
        licenses: HashMap<u32, DsmLicense>
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

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }
    
    pub fn get_classes(&self) -> &HashMap<u32, DsmClasses> {
        &self.classes
    }
    
    pub fn get_subset_version(&self) -> &Option<String> {
        &self.subset_version
    }

    pub fn get_contributor(&self) -> &String {
        &self.contributor
    }

    pub fn get_date_created(&self) -> &String {
        &self.date_created
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_url(&self) -> &String {
        &self.url
    }

    pub fn get_year(&self) -> &String {
        &self.year
    }

    pub fn get_formatter(&self) -> &DsmDataForm {
        &self.formatter
    }

    pub fn get_licenses(&self) -> &HashMap<u32, DsmLicense> {
        &self.licenses
    }
}