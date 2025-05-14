use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MetaData {
    name: String,
    version: u32,
    classes: HashMap<u32, String>,
}

impl MetaData {
    pub fn new(name: String, version: u32, classes: HashMap<u32, String>) -> Self {
        Self {
            name,
            version,
            classes
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }
    
    pub fn get_classes(&self) -> &HashMap<u32, String> {
        &self.classes
    }
}