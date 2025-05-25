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

pub struct DsmMetaDataBuilder {
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

impl DsmMetaDataBuilder {
    pub fn new(
        name: String,
        version: String,
        formatter: DsmDataForm,
        classes: HashMap<u32, DsmClasses>
    ) -> Self {
        Self {
            name,
            version,
            subset_version: None,
            contributor: String::new(),
            date_created: String::new(),
            description: String::new(),
            url: String::new(),
            year: String::new(),
            formatter,
            classes,
            licenses: HashMap::new()
        }
    }

    pub fn set_contributor(mut self, contributor: String) -> Self {
        self.contributor = contributor;
        self
    }

    pub fn set_date_created(mut self, date_created: String) -> Self {
        self.date_created = date_created;
        self
    }

    pub fn set_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn set_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn set_year(mut self, year: String) -> Self {
        self.year = year;
        self
    }

    pub fn set_licenses(mut self, licenses: HashMap<u32, DsmLicense>) -> Self {
        self.licenses = licenses;
        self
    }

    pub fn build(self) -> DsmMetaData {
        DsmMetaData {
            name: self.name,
            version: self.version,
            subset_version: self.subset_version,
            contributor: self.contributor,
            date_created: self.date_created,
            description: self.description,
            url: self.url,
            year: self.year,
            formatter: self.formatter,
            classes: self.classes,
            licenses: self.licenses,
        }
    }
}