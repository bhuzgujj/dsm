use clap::Args;
use interfaces::models::Settings;
use storage::Storage;

/// Show currently stored sets
#[derive(Args, Debug)]
pub struct List {
}

impl List {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		let sets = Storage::Local {
			ledger_directory: settings.get_ledger_path(),
			store_directory: settings.get_store_path(),
		}.list_raw().await?;
		println!("{: <30}{: <20}{: <20}{: <50}", "Set name", "Version", "Image Count", "Classes");
		println!("{:=<30}{:=<20}{:=<20}{:=<50}", "", "", "", "");
		for set in sets {
			let counts = set.get_class_count();
			let classes = serde_json::to_string(&set.get_classes().iter().map(|(k, c)| {
				let count = counts.get(k).unwrap_or(&0);
				if c.get_subclass().clone().is_some_and(|s| !s.is_empty()) {
					format!("{}({}) [{}]", c.get_classes_name().clone(), c.get_subclass().clone().unwrap(), count)
				} else {
					format!("{} [{}]", c.get_classes_name().clone(), count)
				}
			}).collect::<Vec<_>>())?;

			println!(
				"{: <30}{: <20}{: <20}{: <50}", 
				cut_string(set.get_name(), 30), 
				cut_string(set.get_version(), 20), 
				cut_string(&set.get_entry_count().to_string(), 20), 
				classes
			);
		}
		let sets = Storage::Local {
			ledger_directory: settings.get_ledger_path(),
			store_directory: settings.get_store_path(),
		}.list_merged().await?;
		println!();
		println!("{: <30}{: <20}{: <50}", "Set name", "Version", "Sets");
		println!("{:=<30}{:=<20}{:=<50}", "", "", "");
		for merged_set in sets {
			let sets = merged_set.get_datasets_include().iter()
				.map(|s| format!("{}={}", s.get_name(), s.get_version()))
				.collect::<Vec<String>>();
			let sets = serde_json::to_string(&sets)?;
			println!(
				"{: <30}{: <20}{: <50}", 
				cut_string(&merged_set.get_name().to_string(), 30), 
				cut_string(merged_set.get_version(), 20), 
				sets
			);
		}
		Ok(())
	}
}

fn cut_string(string: &String, len: usize) -> String {
	if string.len() >= len {
		format!("{}...", &string[0..26])
	} else {
		string.clone()
	}
}