use crate::models::licence::DsmLicense;
use crate::models::{classes::DsmClasses, DsmLocation};
use entries::DsmEntry;
use metadata::{DsmMetaData, DsmMetaDataBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{ClassMapper, LicenseMapper};

pub mod annotation;
pub mod classes;
pub mod entries;
pub mod licence;
pub mod metadata;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DsmSets {
    metadata: DsmMetaData,
    entries: HashMap<String, Vec<DsmEntry>>,
}

impl DsmSets {
    pub fn new(metadata: DsmMetaData, entries: HashMap<String, Vec<DsmEntry>>) -> DsmSets {
        Self { metadata, entries }
    }

    pub fn get_name(&self) -> &String {
        self.metadata.get_name()
    }

    pub fn get_keyed_name(&self) -> String {
        format!(
            "{}={}",
            self.metadata.get_name(),
            self.metadata.get_version()
        )
    }

    pub fn get_metadata(&self) -> &DsmMetaData {
        &self.metadata
    }

    pub fn get_license(&self) -> &HashMap<u32, DsmLicense> {
        self.metadata.get_licenses()
    }

    pub fn get_version(&self) -> &String {
        self.metadata.get_version()
    }

    pub fn get_mut_entries(&mut self) -> &mut HashMap<String, Vec<DsmEntry>> {
        &mut self.entries
    }

    pub fn get_entries(&self) -> &HashMap<String, Vec<DsmEntry>> {
        &self.entries
    }

    pub fn get_entries_mut(&mut self) -> &mut HashMap<String, Vec<DsmEntry>> {
        &mut self.entries
    }

    pub fn get_classes(&self) -> &HashMap<u32, DsmClasses> {
        self.metadata.get_classes()
    }

    pub fn is_incomplet(&self) -> &bool {
        self.metadata.is_incomplet()
    }

    pub fn get_class_count(&self) -> HashMap<u32, u32> {
        let mut class_count: HashMap<u32, u32> = HashMap::new();
        for (_, entry) in self.entries.iter() {
            for entry in entry {
                for annotation in entry.get_annotation() {
                    if class_count.contains_key(annotation.get_class()) {
                        class_count.insert(
                            *annotation.get_class(),
                            class_count.get(annotation.get_class()).unwrap() + 1,
                        );
                    } else {
                        class_count.insert(*annotation.get_class(), 1);
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

    pub fn remap(&self, map: &ClassMapper) -> anyhow::Result<Self> {
        let final_map = self.get_classes();
        let licence_mapper = LicenseMapper::from(self);
        let dataset_name = self.get_name();
        let mut new_entries = HashMap::new();
        for (name, entries) in self.get_entries() {
            let mut subset = Vec::new();
            for entry in entries {
                subset.push(entry.remap(dataset_name, map, &licence_mapper, final_map)?);
            }
            new_entries.insert(name.clone(), subset);
        }
        Ok(Self {
            metadata: DsmMetaDataBuilder::from(self.metadata.clone())
                .set_classes(map.get_dsm_classes())
                .build(),
            entries: new_entries,
        })
    }

    pub fn get_rel_from_storage(&self) -> String {
        format!("{}/{}", self.get_name(), self.get_version())
    }

    pub fn update_location(&mut self, save_location: DsmLocation) {
        for (_, entries) in self.entries.iter_mut() {
            for entry in entries.iter_mut() {
                entry.update_location(&save_location);
            }
        }
    }
}
