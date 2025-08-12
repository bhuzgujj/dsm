use std::collections::HashMap;

use interfaces::log_err;
use interfaces::models::datasets::MetaDataBuilder;
use interfaces::models::{datasets::Dataset, ClassMapper, MergedSet, Settings};
use serde::{Deserialize, Serialize};
use storage::Storage;
use tauri::async_runtime::Mutex;
use tauri::{Error, State};

#[derive(Deserialize, Serialize, Clone)]
pub struct IncludedSet {
	names: String,
	version: String,
	group: String,
}

#[tauri::command]
pub async fn merge_new(
	state: State<'_, Mutex<Settings>>,
	name: String,
	version: String,
	included: Vec<IncludedSet>,
	class_mapping: ClassMapper,
) -> Result<(), Error> {
	let state = state.lock().await;
	let mut datasets = HashMap::new();
	let storage = Storage::Local {
		ledger_directory: state.ledger_path(),
		store_directory: state.store_path(),
		action_log_limit: state.action_count(),
	};
	for include in included {
		let local_set: Option<Dataset> = storage
			.read(include.names.clone(), include.version.clone())
			.await?;
		let sets_label = format!("{}~{}", &include.names, &include.version);
		if let Some(local_set) = local_set {
			datasets.insert(sets_label, (include.group, local_set));
		} else {
			let any: anyhow::Result<()> = log_err!(format!("Could not find {}", sets_label));
			match any {
				Ok(_) => return Ok(()),
				Err(err) => return Err(Error::Anyhow(err)),
			}
		}
	}
	let merge_set = MergedSet::new(
		datasets.clone(),
		name.clone(),
		version.clone(),
		class_mapping,
	);
	storage.ledge(&merge_set).await?;

	for (_, set) in datasets.values() {
		let merged_key = format!("{}~{}", merge_set.name(), merge_set.version());
		let builder =
			MetaDataBuilder::from(set.metadata().to_owned()).add_contained_in_merged(merged_key);
		storage
			.ledge(&Dataset::new(builder.build(), set.entries().to_owned()))
			.await?;
	}
	Ok(())
}
