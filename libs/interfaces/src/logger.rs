use crate::models::Settings;
use crate::paths::dsm_dir;
use chrono::Local;
use log::{Log, Metadata, Record};
use std::fmt::format;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

const FILE_NAME: &'static str = "dsm.log";

pub fn refresh(settings: &Settings) {
    log::set_max_level(settings.get_log_level());
    #[allow(static_mut_refs)]
    unsafe {
        LOGGER.file = Some(dsm_dir().join(FILE_NAME));
    }
}

/// This method WILL unwrap errors
/// It uses unsafe!
///
/// DO NOT CATCH THE ERROR
pub fn bind_logger(settings: &Settings) {
    refresh(settings);

    #[allow(static_mut_refs)]
    unsafe {
        log::set_logger(&LOGGER).unwrap();
    }
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

        let log_line = log(&record);
        println!("{}", log_line);

        if let Some(file_path) = &self.file {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_path)
                .unwrap();
            file.write(format!("{}\n", log_line).as_bytes())
                .expect("Could not write to the log file");
        }
    }

    fn flush(&self) {}
}

fn log(record: &Record) -> String {
    let local = Local::now();
    format!(
        "[{}] [{}] {} ({}:{}): {}",
        local.format("%Y-%m-%d %H:%M:%S.%.3f"),
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

static mut LOGGER: Logger = Logger { file: None };
