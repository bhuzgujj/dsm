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
}

impl Into<DataForm> for Custom {
	fn into(self) -> DataForm {
		DataForm::Custom(self.formats.clone())
	}
}

impl Custom {
	pub async fn execute(&self, _settings: &Settings) -> anyhow::Result<()> {
		todo!("Custom formatter not implemented yet");
	}
}