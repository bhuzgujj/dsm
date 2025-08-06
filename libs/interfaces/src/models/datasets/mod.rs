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
    pub metadata: DsmMetaData,
    pub entries: HashMap<String, Vec<DsmEntry>>,
}

impl DsmSets {
    pub fn new(metadata: DsmMetaData, entries: HashMap<String, Vec<DsmEntry>>) -> DsmSets {
        Self { metadata, entries }
    }

    pub fn name(&self) -> &String {
        self.metadata.name()
    }

    pub fn keyed_name(&self) -> String {
        format!("{}={}", self.metadata.name(), self.metadata.version())
    }

    pub fn metadata(&self) -> &DsmMetaData {
        &self.metadata
    }

    pub fn licenses(&self) -> &HashMap<u32, DsmLicense> {
        self.metadata.licenses()
    }

    pub fn version(&self) -> &String {
        self.metadata.version()
    }

    pub fn entries_mut(&mut self) -> &mut HashMap<String, Vec<DsmEntry>> {
        &mut self.entries
    }

    pub fn entries(&self) -> &HashMap<String, Vec<DsmEntry>> {
        &self.entries
    }

    pub fn classes(&self) -> &HashMap<u32, DsmClasses> {
        self.metadata.classes()
    }

    pub fn is_incomplet(&self) -> &bool {
        self.metadata.is_incomplet()
    }

    pub fn class_count(&self) -> HashMap<u32, u32> {
        let mut class_count: HashMap<u32, u32> = HashMap::new();
        for (_, entry) in self.entries.iter() {
            for entry in entry {
                for annotation in entry.annotation() {
                    if class_count.contains_key(annotation.class()) {
                        class_count.insert(
                            *annotation.class(),
                            class_count.get(annotation.class()).unwrap() + 1,
                        );
                    } else {
                        class_count.insert(*annotation.class(), 1);
                    }
                }
            }
        }
        class_count
    }

    pub fn entry_count(&self) -> u32 {
        let mut class_count: u32 = 0;
        for (_, entry) in self.entries.iter() {
            for _ in entry {
                class_count += 1
            }
        }
        class_count
    }

    pub fn remap(&self, map: &ClassMapper) -> anyhow::Result<Self> {
        let final_map = self.classes();
        let licence_mapper = LicenseMapper::from(self);
        let dataset_name = self.name();
        let mut new_entries = HashMap::new();
        for (name, entries) in self.entries() {
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

    pub fn update_location(&mut self, save_location: DsmLocation) {
        for (_, entries) in self.entries.iter_mut() {
            for entry in entries.iter_mut() {
                entry.update_location(&save_location);
            }
        }
    }
}
