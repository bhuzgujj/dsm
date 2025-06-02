use std::collections::HashMap;

use chrono::{DateTime, Utc};
use interfaces::models::{metadata::DsmMetaDataBuilder, DsmSets};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DataAssets {
    creation_context: CreationContext, 
    latest_version: String,
    name: String,
    properties: HashMap<String, String>,
    tags: HashMap<String, String>,
    #[serde(rename = "type")]
    types: String
}

impl DataAssets {
    pub fn to_dsm(&self) -> DsmSets {
        DsmSets::new(
            DsmMetaDataBuilder::new(
                self.name.clone(), 
                self.latest_version.clone(), 
                interfaces::models::DsmDataForm::Unknown, 
                HashMap::new()
            )
                .set_incomplete(true)
                .build(), 
            HashMap::new()
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreationContext {
    created_at: DateTime<Utc>,
    created_by: String,
    created_by_type: String,
    last_modified_at: DateTime<Utc>
}