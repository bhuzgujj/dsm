use crate::models::Settings;
use crate::paths::dsm_dir;
use chrono::Local;
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
    create_dir_all(dir)?;
    log::set_max_level(settings.get_log_level());
    #[allow(static_mut_refs)]
    unsafe {
        LOGGER.file = Some(dsm_dir().join(FILE_NAME));
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
        log::set_logger(&LOGGER).unwrap();
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
        println!("{}", record.args());

        if let Some(file_path) = &self.file {
            let mut file = OpenOptions::new()
                
                .append(true)
                .open(file_path)
                .unwrap();
            file.write_all(format!("{}\n", log_line).as_bytes())
                .expect("Could not write to the log file");
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
