use clap::Args;
use interfaces::{log_err, models::Settings};

/// Mutate the metadata of a dataset
#[derive(Args, Debug)]
pub struct Dsm {
	/// The version of the dataset
	name: String,

	/// The version of the dataset
	version: String,

    /// The location of the datasets, this will send it to the new location
    #[clap(long, short)]
    location: Option<String>,

}
impl Dsm {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
        log_err!("Not implemented yet")
    }
}