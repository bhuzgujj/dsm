use crate::format::Format;
use clap::Args;
use interfaces::models::Settings;
use log::debug;
use serializer::DataForm;
use std::path::PathBuf;
use storage::Storage;

/// Parse the files in a known standard format and store the set in a storage
#[derive(Args, Debug)]
pub struct Store {
    /// Root directory of the files
    path: String,

    /// In which format the files will be read as.
    /// If it is not a known standard, it will pick a script in scripts directory.
    /// Standard supported:
    ///  - coco-1-0
    ///  - yolo-1-1
    #[clap(short, long, verbatim_doc_comment)]
    formats: String,

    /// Datasets registered name
    #[clap(short, long)]
    name: Option<String>,

    /// Datasets registered version
    #[clap(short, long, default_value = "1")]
    version: String,
}

impl Store {
    pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
        let path = PathBuf::from(self.path.clone());
        let formatter: DataForm = Format::from(self.formats.clone()).into();
        debug!("Storing dataset '{}' with '{}'", path.display(), formatter);
        let datasets = formatter.read(&path, self.name.clone(), self.version.clone())?;
        let storage = Storage::local(settings);
        for dataset in datasets.iter() {
            storage.store(dataset, &path).await?;
            storage.ledge(dataset).await?;
        }
        Ok(())
    }
}
