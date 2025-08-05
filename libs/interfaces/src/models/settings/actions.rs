use anyhow;
use chrono::{DateTime, Utc};
use std::{collections::VecDeque, fs::read_to_string};

use serde::{Deserialize, Serialize};

use crate::paths::{dsm_dir, write_to_file};

const FILENAME: &str = "actions_logs.json";

pub fn get_actions(limit: usize) -> VecDeque<Action> {
    if limit == 0 {
        return VecDeque::new();
    }
    let filename = dsm_dir().join(FILENAME);
    if let Ok(file_ctnt) = read_to_string(&filename) {
        let mut actions = serde_json::from_str(file_ctnt.as_str()).unwrap_or(VecDeque::new());
        actions.truncate(limit);
        actions
    } else {
        VecDeque::new()
    }
}

pub fn add_action(action: Action, limit: usize) -> anyhow::Result<()> {
    if limit > 0 {
        let filename = dsm_dir().join(FILENAME);
        let mut actions = get_actions(limit);
        actions.push_front(action);
        let content = serde_json::to_string(&actions)?;
        write_to_file(&filename, content, true, true)?;
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    name: String,
    version: String,
    datasets: Vec<SetInfo>,
    action: String,
    time: DateTime<Utc>,
}

impl Action {
    pub fn store(name: String, version: String) -> Self {
        Self {
            name,
            version,
            datasets: Vec::new(),
            action: "New".to_string(),
            time: Utc::now(),
        }
    }

    pub fn merge(name: String, version: String, datasets: Vec<SetInfo>) -> Self {
        Self {
            name,
            version,
            datasets,
            action: "Merge".to_string(),
            time: Utc::now(),
        }
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn datasets(&self) -> &[SetInfo] {
        &self.datasets
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn time(&self) -> DateTime<Utc> {
        self.time
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetInfo {
    name: String,
    version: String,
}

impl SetInfo {
    pub fn new(name: String, version: String) -> Self {
        Self { name, version }
    }
}
