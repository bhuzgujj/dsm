use crate::format::Format;
use clap::Args;
use interfaces::models::Settings;
use serializer::DataForm;
use std::path::PathBuf;
use storage::Storage;

/// Parse the files in a known standard format
#[derive(Args, Debug)]
pub struct Std {
    /// In which format the files will be read as
    formats: Format,

    /// Root directory of the files
    path: String,

    /// Datasets registered name
    #[clap(long)]
    name: Option<String>,

    /// Datasets registered version
    #[clap(short, long)]
    version: Option<u32>,
}

impl Std {
    pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
        let path = PathBuf::from(self.path.clone());
        let formatter: DataForm = self.formats.clone().into();
        let datasets = formatter.read(&path, self.name.clone(), self.version)?;
        for dataset in datasets.iter() {
            Storage::Local {
                ledger_directory: settings.get_ledger_path(),
                store_directory: settings.get_store_path(),
            }.store(dataset, &path).await?;
        }
        Ok(())
    }
}
