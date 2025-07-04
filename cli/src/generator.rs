use crate::format::Format;
use clap::Args;
use interfaces::{
    log_err,
    models::{MergedSet, Settings},
};
use serializer::DataForm;
use std::path::PathBuf;
use storage::Storage;

/// Create a direvative dataset from storage in a known format
#[derive(Args, Debug)]
pub struct Generator {
    /// In which format the files will be read as
    formats: String,

    /// Root directory of the files
    path: String,

    /// Datasets to include
    #[clap(short, long)]
    datasets: String,

    /// Datasets registered version
    #[clap(short, long)]
    version: String,
}

impl Generator {
    pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
        if self.datasets.is_empty() {
            return log_err!("Require at least one dataset");
        }
        let formatter: DataForm = Format::from(self.formats.clone()).into();
        let storage = Storage::Local {
            ledger_directory: settings.get_ledger_path(),
            store_directory: settings.get_store_path(),
        };
        let datasets = storage
            .read(self.datasets.clone(), self.version.clone())
            .await?;
        let (new_set, image_rel_path_mapping) = if let Some(dsm_set) = datasets {
            (dsm_set, None)
        } else {
            let merged_set: MergedSet = match storage
                .read(self.datasets.clone(), self.version.clone())
                .await?
            {
                Some(val) => val,
                None => return log_err!("Could not find datasets".to_string()),
            };

            merged_set.to_dataset(formatter.to_data_form())?
        };
        let path = PathBuf::from(&self.path);
        formatter.write(
            &path,
            &settings.get_store_path(),
            &new_set,
            image_rel_path_mapping,
        )?;
        Ok(())
    }
}
