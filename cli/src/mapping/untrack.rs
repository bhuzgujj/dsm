use std::{fs::read_to_string, path::PathBuf};

use clap::Args;
use interfaces::{
    log_err,
    models::{ClassMapper, Settings},
};
use serializer::DataForm;

use crate::format::Format;

#[derive(Debug, Args)]
pub struct Untrack {
    #[clap(long)]
    input_path: PathBuf,

    #[clap(long)]
    input_format: String,

    #[clap(long)]
    output_path: PathBuf,

    #[clap(long)]
    output_format: String,

    /// Datasets registered version
    #[clap(short, long, default_value = "1")]
    version: String,

    #[clap(short, long)]
    mapping_file: PathBuf,
}

impl Untrack {
    pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
        let _ = settings;
        let iformat: DataForm = Format::from(self.input_format.clone()).into();
        let content = match read_to_string(&self.mapping_file) {
            Ok(content) => content,
            Err(err) => {
                return log_err!(format!(
                    "Could not read file '{}': {err}",
                    &self.mapping_file.display()
                ))
            }
        };
        let mapping: ClassMapper = match toml::from_str(&content) {
            Ok(content) => content,
            Err(err) => {
                return log_err!(format!(
                    "Could not deserialize toml mapping file '{}': {err}",
                    &self.mapping_file.display()
                ))
            }
        };
        let datasets = iformat.read(&self.input_path, None, self.version.clone())?;
        for dataset in datasets {
            let new_ds = dataset.remap(&mapping)?;
            let oformat: DataForm = Format::from(self.input_format.clone()).into();
            oformat.write(&self.output_path, &self.input_path, &new_ds, None)?;
        }
        Ok(())
    }
}
