use clap::Args;
use interfaces::models::{DsmSets, MergedSet, Settings};
use storage::Storage;
use colored::Colorize;

/// Show currently stored sets
#[derive(Args, Debug)]
pub struct List {
    #[clap(long, short, default_value = "false")]
    azure_included: bool,
}

impl List {
    pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
        let mut sets: Vec<DsmSets> = Storage::Local {
            ledger_directory: settings.get_ledger_path(),
            store_directory: settings.get_store_path(),
        }
        .list()
        .await?;

        if self.azure_included {
            let azure_sets: Vec<DsmSets> = Storage::Remote {
                service: settings
                    .get_remote(String::from("my_account"))
                    .unwrap()
                    .clone(),
            }
            .list()
            .await?;
            for set in azure_sets {
                sets.push(set);
            }
        }

        println!(
            "{: <30}{: <20}{: <20}{: <50}",
            "Set name", "Version", "Image Count", "Classes"
        );
        println!("{:=<30}{:=<20}{:=<20}{:=<50}", "", "", "", "");
        for set in sets {
            let counts = set.get_class_count();
            let classes = serde_json::to_string(
                &set.get_classes()
                    .iter()
                    .map(|(k, c)| {
                        let count = counts.get(k).unwrap_or(&0);
                        if c.get_subclass().clone().is_some_and(|s| !s.is_empty()) {
                            format!(
                                "{}({}) [{}]",
                                c.get_classes_name().clone(),
                                c.get_subclass().clone().unwrap(),
                                count
                            )
                        } else {
                            format!("{} [{}]", c.get_classes_name().clone(), count)
                        }
                    })
                    .collect::<Vec<_>>(),
            )?;
			
            if *set.is_incomplet() {
                println!(
                    "{: <30}{: <20}{: <20}{: <50}",
                    cut_string(set.get_name(), 30).as_str().blue(),
                    cut_string(set.get_version(), 20).as_str().blue(),
                    cut_string(&set.get_entry_count().to_string(), 20).as_str().blue(),
                    classes.as_str().blue()
                );
            } else {
                println!(
                    "{: <30}{: <20}{: <20}{: <50}",
                    cut_string(set.get_name(), 30),
                    cut_string(set.get_version(), 20),
                    cut_string(&set.get_entry_count().to_string(), 20),
                    classes
                );
            }
        }
        let sets: Vec<MergedSet> = Storage::Local {
            ledger_directory: settings.get_ledger_path(),
            store_directory: settings.get_store_path(),
        }
        .list()
        .await?;
        println!();
        println!("{: <30}{: <20}{: <50}", "Set name", "Version", "Sets");
        println!("{:=<30}{:=<20}{:=<50}", "", "", "");
        for merged_set in sets {
            let sets = merged_set
                .get_datasets_include()
                .iter()
                .map(|s| format!("{}~{}", s.get_name(), s.get_version()))
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
        format!("{}...", &string[0..len-4])
    } else {
        string.clone()
    }
}
