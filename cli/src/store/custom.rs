use clap::Args;
use interfaces::models::Settings;
use serializer::DataForm;

/// Use a custom script to parse the files (NOT IMPLEMENTED)
#[derive(Args, Debug)]
pub struct Custom {
    /// The script name in which this will be parsed
    formats: String,

    /// Root directory of the files
    path: String,

    /// Datasets registered name
    #[clap(long)]
    name: Option<String>,

    /// Datasets registered version
    #[clap(short, long, default_value_t = 1)]
    version: u32,
}

impl Into<DataForm> for Custom {
    fn into(self) -> DataForm {
        DataForm::Custom(self.formats.clone())
    }
}

impl Custom {
    pub async fn execute(&self, _settings: &Settings) -> anyhow::Result<()> {
        todo!();
        // let path = PathBuf::from(self.path.clone());
        // let name = self.name.clone().unwrap_or(
        //     path.file_name()
        //         .expect("Could not get the directory name")
        //         .to_string_lossy()
        //         .into(),
        // );
        // let datasets = Formatter::Custom(self.formats.clone()).read(&path, name, self.version)?;
        // Ok(())
    }
}
