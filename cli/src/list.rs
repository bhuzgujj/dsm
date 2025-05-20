use clap::Subcommand;
use interfaces::models::Settings;
use storage::Storage;

/// Show currently stored sets
#[derive(Subcommand, Debug)]
pub enum List {
	/// Show raw sets
	Raw,

	/// Show merged sets
	Merged
}

impl List {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		match self {
			List::Raw => {
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
							format!("{}({}) [{}]", c.get_classes().clone(), c.get_subclass().clone().unwrap(), count)
						} else {
							format!("{} [{}]", c.get_classes().clone(), count)
						}
					}).collect::<Vec<_>>())?;

					println!("{: <30}{: <20}{: <20}{: <50}", set.get_name(), set.get_version(), set.get_entry_count(), classes);
				}
			}
			List::Merged => {
				let sets = Storage::Local {
					ledger_directory: settings.get_ledger_path(),
					store_directory: settings.get_store_path(),
				}.list_merged().await?;
				println!("{: <30}{: <20}{: <50}", "Set name", "Version", "Sets");
				println!("{:=<30}{:=<20}{:=<50}", "", "", "");
				for merged_set in sets {
					let sets = merged_set.get_datasets_include().iter()
						.map(|s| format!("{}={}", s.get_name(), s.get_version()))
						.collect::<Vec<String>>();
					let sets = serde_json::to_string(&sets)?;
					println!("{: <30}{: <20}{: <50}", merged_set.get_name(), merged_set.get_version(), sets);
				}
			}
		}
		Ok(())
	}
}