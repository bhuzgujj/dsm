use clap::Args;
use colored::Colorize;
use interfaces::models::{DsmSets, MergedSet, Settings};
use storage::Storage;

/// Show currently stored sets
#[derive(Args, Debug)]
pub struct List {
    /// Add azure's sets
    #[clap(long, default_value = "false")]
    azure_included: bool,
}

impl List {
    pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
        let mut sets: Vec<DsmSets> = Storage::local(settings).list().await?;

        if self.azure_included {
            let azure_sets: Vec<DsmSets> = Storage::Remote {
                service: settings
                    .remote(String::from("my_account"))
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
            let counts = set.class_count();
            let classes = serde_json::to_string(
                &set.classes()
                    .iter()
                    .map(|(k, c)| {
                        let count = counts.get(k).unwrap_or(&0);
                        if c.subclass().clone().is_some_and(|s| !s.is_empty()) {
                            format!(
                                "{}({}) [{}]",
                                c.class().clone(),
                                c.subclass().clone().unwrap(),
                                count
                            )
                        } else {
                            format!("{} [{}]", c.class().clone(), count)
                        }
                    })
                    .collect::<Vec<_>>(),
            )?;

            if *set.is_incomplet() {
                println!(
                    "{: <30}{: <20}{: <20}{: <50}",
                    cut_string(set.name(), 30).as_str().blue(),
                    cut_string(set.version(), 20).as_str().blue(),
                    cut_string(&set.entry_count().to_string(), 20)
                        .as_str()
                        .blue(),
                    classes.as_str().blue()
                );
            } else {
                println!(
                    "{: <30}{: <20}{: <20}{: <50}",
                    cut_string(set.name(), 30),
                    cut_string(set.version(), 20),
                    cut_string(&set.entry_count().to_string(), 20),
                    classes
                );
            }
        }
        let sets: Vec<MergedSet> = Storage::local(settings).list().await?;
        println!();
        println!("{: <30}{: <20}{: <50}", "Set name", "Version", "Sets");
        println!("{:=<30}{:=<20}{:=<50}", "", "", "");
        for merged_set in sets {
            let sets = merged_set
                .datasets_include()
                .iter()
                .flat_map(|(_, s)| s)
                .map(|s| format!("{}~{}", s.name(), s.version()))
                .collect::<Vec<String>>();
            let sets = serde_json::to_string(&sets)?;
            println!(
                "{: <30}{: <20}{: <50}",
                cut_string(merged_set.name(), 30),
                cut_string(merged_set.version(), 20),
                sets
            );
        }
        Ok(())
    }
}

fn cut_string(string: &str, len: usize) -> String {
    if string.len() >= len {
        format!("{}...", &string[0..len - 4])
    } else {
        string.to_owned()
    }
}
