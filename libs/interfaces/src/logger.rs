use crate::models::Settings;
use crate::paths::dsm_dir;
use anyhow::anyhow;
use chrono::Local;
use colored::Colorize;
use log::{trace, Log, Metadata, Record};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

const FILE_NAME: &str = "dsm.log";
static mut LOGGER: Logger = Logger { file: None };

/// This function modify the logger's field
///
/// This function mutate global state!
pub fn refresh(settings: &Settings) -> anyhow::Result<()> {
	let dir = dsm_dir();
	if let Err(err) = create_dir_all(&dir) {
		let msg = format!("Failed to create {} directory: {err}", dir.display()).red();
		println!("{msg}");
		return Err(anyhow!(msg));
	}

	let log_file_path = dsm_dir().join(FILE_NAME);
	if let Err(err) = OpenOptions::new()
		.create(true)
		.truncate(false)
		.write(true)
		.open(&log_file_path)
	{
		let msg = format!("Failed to create {}: {err}", &log_file_path.display()).red();
		println!("{msg}");
		return Err(anyhow!(msg));
	}
	log::set_max_level(settings.log_level());
	#[allow(static_mut_refs)]
	unsafe {
		LOGGER.file = Some(log_file_path);
	}
	Ok(())
}

/// This function binds the logger and refresh the settings
///
/// This function mutate global state!
pub fn bind_logger(settings: &Settings) -> anyhow::Result<()> {
	refresh(settings)?;

	#[allow(static_mut_refs)]
	unsafe {
		if let Err(err) = log::set_logger(&LOGGER) {
			let msg = format!("Failed set logger: {err}").red();
			println!("{msg}");
			return Err(anyhow!(msg));
		}
	}
	trace!("Logger bound!");
	Ok(())
}

struct Logger {
	file: Option<PathBuf>,
}

impl Log for Logger {
	fn enabled(&self, _metadata: &Metadata) -> bool {
		true
	}

	fn log(&self, record: &Record) {
		if !self.enabled(record.metadata()) {
			return;
		}

		let log_line = log(record);
		if let Some(file_path) = &self.file {
			let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
			file.write_all(format!("{log_line}\n").as_bytes())
				.expect("Could not write to the log file");
		} else {
			println!("{log_line}");
		}
	}

	fn flush(&self) {}
}

fn log(record: &Record) -> String {
	let local = Local::now();
	format!(
		"[{}] [{}] {} ({}{}): {}",
		local.format("%Y-%m-%d %H:%M:%S%.3f"),
		record.level(),
		record.module_path().unwrap_or("<Unknown>"),
		record.file().unwrap_or("<Unknown>"),
		if let Some(line) = record.line() {
			format!(":{line}")
		} else {
			String::new()
		},
		record.args()
	)
}

/// This macro Log and return an anyhow error of the type of the caller
///
/// Usage:
/// ```
/// use interfaces::log_err;
///
/// fn errored(value: String) -> anyhow::Result<String> {
///     if value.is_empty() {
///         return log_err!("Value must not be empty");
///     }
///     return Ok(value)
/// }
/// ```
///
#[macro_export]
macro_rules! log_err {
	($message:expr) => {{
		log::error!("{}", $message);
		Err(anyhow::anyhow!($message))
	}};
}
