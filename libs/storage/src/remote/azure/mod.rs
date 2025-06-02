mod access_token;
mod wrapper;
mod assets;

pub use wrapper::AzureClientWrapper;

use crate::storable::Storable;

pub(crate) async fn read<T: Storable>(
    _azure_client: &mut AzureClientWrapper,
    _storage_container: String,
    _name: String,
    _version: String,
) -> anyhow::Result<Option<T>> {
    Ok(None)
}

pub(crate) async fn list<T: Storable>(
    azure_client: &mut AzureClientWrapper,
) -> anyhow::Result<Vec<T>> {
    azure_client.list_data_assets().await
}
