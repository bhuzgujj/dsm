use entries::DsmEntry;
use metadata::DsmMetaData;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::classes::DsmClasses;
use crate::models::licence::DsmLicense;

pub mod entries;
pub mod metadata;
pub mod annotation;
pub mod licence;
pub mod classes;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DsmSets {
    metadata: DsmMetaData,
    entries: HashMap<String, Vec<DsmEntry>>,
}

impl DsmSets {
    pub fn new(metadata: DsmMetaData, entries: HashMap<String, Vec<DsmEntry>>) -> DsmSets {
        Self { metadata, entries }
    }
    
    pub fn get_name(&self) -> String {
        self.metadata.get_name()
    }

    pub fn get_metadata(&self) -> &DsmMetaData {
        &self.metadata
    }

    pub fn get_license(&self) -> &HashMap<u32, DsmLicense> {
        self.metadata.get_licenses()
    }

    pub fn get_version(&self) -> u32 {
        self.metadata.get_version()
    }

    pub fn get_mut_entries(&mut self) -> &mut HashMap<String, Vec<DsmEntry>> {
        &mut self.entries
    }

    pub fn get_entries(&self) -> &HashMap<String, Vec<DsmEntry>> {
        &self.entries
    }
    
    pub fn get_classes(&self) -> HashMap<u32, DsmClasses> {
        self.metadata.get_classes()
    }

    pub fn get_class_count(&self) -> HashMap<u32, u32> {
        let mut class_count: HashMap<u32, u32> = HashMap::new();
        for (_, entry) in self.entries.iter() {
            for entry in entry {
                for annotation in entry.get_annotation() {
                    if class_count.contains_key(&annotation.get_class()) {
                        class_count.insert(annotation.get_class(), class_count.get(&annotation.get_class()).unwrap() + 1);
                    } else {
                        class_count.insert(annotation.get_class(), 1);
                    }
                }
            }
        }
        class_count
    }

    pub fn get_entry_count(&self) -> u32 {
        let mut class_count: u32 = 0;
        for (_, entry) in self.entries.iter() {
            for _ in entry {
                class_count += 1
            }
        }
        class_count
    }
}