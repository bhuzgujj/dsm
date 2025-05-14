use entries::DatasetEntry;
use metadata::MetaData;
use std::collections::HashMap;

pub mod entries;
pub mod metadata;
pub mod annotation;

#[derive(Debug, Clone)]
pub struct Datasets {
    metadata: MetaData,
    entries: HashMap<String, Vec<DatasetEntry>>,
}

impl Datasets {
    pub fn new(metadata: MetaData, entries: HashMap<String, Vec<DatasetEntry>>) -> Datasets {
        Self { metadata, entries }
    }
    
    pub fn get_name(&self) -> String {
        self.metadata.get_name()
    }

    pub fn get_version(&self) -> u32 {
        self.metadata.get_version()
    }

    pub fn get_mut_entries(&mut self) -> &mut HashMap<String, Vec<DatasetEntry>> {
        &mut self.entries
    }

    pub fn get_entries(&self) -> &HashMap<String, Vec<DatasetEntry>> {
        &self.entries
    }
    
    pub fn get_classes(&self) -> &HashMap<u32, String> {
        self.metadata.get_classes()
    }
}
