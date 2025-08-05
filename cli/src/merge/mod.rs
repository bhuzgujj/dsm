mod add;
mod new;

use crate::format::Format;
use crate::merge::add::Add;
use crate::merge::new::New;
use clap::Subcommand;
use interfaces::log_err;
use interfaces::models::{DsmSets, Settings};
use serializer::DataForm;
use std::collections::HashMap;
use std::path::PathBuf;
use storage::Storage;

/// Manage merged sets
#[derive(Subcommand, Debug)]
pub enum Merge {
    New(New),
    Add(Add),
}

impl Merge {
    pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
        match self {
            Merge::New(new) => new.execute(settings).await,
            Merge::Add(add) => add.execute(settings).await,
        }
    }
}

pub async fn extract_dsm_from_storage(
    sets_keys: &[String],
    datasets: &mut HashMap<String, (String, DsmSets)>,
    storage: &Storage,
) -> anyhow::Result<()> {
    for sets_label in sets_keys.iter() {
        if datasets.contains_key(sets_label) {
            return log_err!(format!("Cannot have twice the same dataset {}", sets_label));
        }
        let splits: Vec<&str> = sets_label.split('=').collect();
        if splits.len() != 3 {
            return log_err!(format!("Can only have 2 equals sign in {}", sets_label));
        }
        let name = splits[0];
        let version = splits[1];
        let group = splits[2];
        let local_set: Option<DsmSets> =
            storage.read(name.to_string(), version.to_string()).await?;
        if let Some(local_set) = local_set {
            datasets.insert(sets_label.clone(), (group.to_string(), local_set));
        } else {
            return log_err!(format!("Could not find {}", sets_label));
        }
    }
    Ok(())
}

pub async fn extract_dsm_from_path(
    paths: &Vec<String>,
    datasets: &mut HashMap<String, (String, DsmSets)>,
    storage: &Storage,
    settings: &Settings,
) -> anyhow::Result<()> {
    for path in paths {
        let splits: Vec<&str> = path.split(':').collect();
        if splits.len() != 3 {
            return log_err!(format!("Can only have 2 equals sign in {}", path));
        }
        let format: DataForm = Format::try_from(splits[0].trim().to_string())?.into();
        let dataset_path = PathBuf::from(splits[1].trim());
        let group = splits[2];
        let dataset = format.read(
            &dataset_path,
            None,
            "0".to_string(),
            settings.interpreters(),
        )?;
        for ds in dataset {
            storage.store(&ds, &dataset_path).await?;
            storage.ledge(&ds).await?;
            datasets.insert(ds.get_name().clone(), (group.to_string(), ds));
        }
    }
    Ok(())
}
