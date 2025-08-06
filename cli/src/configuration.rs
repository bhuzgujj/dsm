use clap::Args;
use interfaces::models::Settings;
use log::LevelFilter;
use std::str::FromStr;

/// Modify global settings
#[derive(Args, Debug)]
pub struct Configuration {
	/// Set global log level
	#[clap(short, long)]
	log_level: Option<String>,
}

impl Configuration {
	pub fn configure(&self, settings: &mut Settings) -> anyhow::Result<()> {
		if let Some(log_level) = &self.log_level {
			let level = LevelFilter::from_str(log_level)
				.unwrap_or_else(|_| panic!("Invalid log level: {}", log_level));
			settings.set_log_level(level);
		}
		Ok(())
	}
}
