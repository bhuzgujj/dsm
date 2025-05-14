mod store;
mod format;
mod configuration;

use std::fs::{remove_dir, remove_dir_all};
use std::path::PathBuf;
use clap::Parser;
use log::error;

use interfaces::logger;
use interfaces::models::Settings;
use serializer::Formatter;
use storage::Storage;

use crate::configuration::Configuration;
use crate::store::Store;

#[derive(Parser, Debug)]
enum Cli {
    #[command(subcommand)]
    Store(Store),
    Config(Configuration),
    Test
}

fn wrapper() -> anyhow::Result<()> {
    let mut settings = Settings::load();
    logger::bind_logger(&settings)?;
    Storage::Local(settings.get_dataset_database()).initialize()?;
    match Cli::parse() {
        Cli::Store(store) => store.execute(&settings)?,
        Cli::Config(configuration) => configuration.configure(&mut settings)?,
        Cli::Test => {
            let datasets = Storage::Local(settings.get_dataset_database()).read("drone-man-yolo".to_string(), 1)?;
            dbg!(&datasets);
            let output = PathBuf::from("./tmp/output");
            let _ = remove_dir_all(&output);
            Formatter::Yolo1_1.write(&output, &settings.get_image_store(), &datasets)?;
        },
    };
    settings.save()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    match wrapper() {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("Error: {}", e);
            Err(e)
        }
    }
}