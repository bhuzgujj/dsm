use clap::Args;
use interfaces::models::{actions::get_actions, Settings};

/// Action recently done
#[derive(Args, Debug)]
pub struct Actions {}

impl Actions {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		println!(
			"{: <10}{: <30}{: <20}{: <30}{: <50}",
			"Action", "Name", "Version", "Time", "Other"
		);
		println!("{:=<10}{:=<30}{:=<20}{:=<30}{:=<50}", "", "", "", "", "");
		let actions = get_actions(settings.action_count());
		if actions.is_empty() {
			println!("No actions...");
		} else {
			for action in actions {
				let sets = serde_json::to_string(&action.datasets())?;
				println!(
					"{: <10}{: <30}{: <20}{: <30}{: <50}",
					action.action(),
					cut_string(action.name().to_string(), 30),
					cut_string(action.version().to_string(), 20),
					action.time().format("%Y-%m-%d %H:%M:%S"),
					sets
				);
			}
		}
		Ok(())
	}
}

fn cut_string(string: String, len: usize) -> String {
	if string.len() >= len {
		format!("{}...", &string[0..len - 4])
	} else {
		string.clone()
	}
}
