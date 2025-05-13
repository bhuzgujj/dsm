use std::collections::HashMap;

pub struct MetaData {
    name: String,
    version: u32,
    classes: HashMap<u32, String>,
}
