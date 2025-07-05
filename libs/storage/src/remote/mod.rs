use azure::AzureClientWrapper;
use interfaces::models::remotes::Remote;

use crate::storable::Storable;

mod azure;

pub(crate) async fn read<T: Storable>(
    remote: &Remote,
    name: String,
    version: String,
) -> anyhow::Result<Option<T>> {
    match remote {
        Remote::Azure {
            storage_container,
            storage_account,
        } => {
            azure::read(
                &mut AzureClientWrapper::new(storage_account.clone()),
                storage_container.clone(),
                name,
                version,
            )
            .await
        }
    }
}

pub(crate) async fn list<T: Storable>(remote: &Remote) -> anyhow::Result<Vec<T>> {
    match remote {
        Remote::Azure {
            storage_container: _storage_container,
            storage_account,
        } => {
            azure::list(&mut AzureClientWrapper::new(storage_account.clone())).await
        }
    }
}

pub(crate) async fn store<T: Storable>(remote: &Remote, _storable: &T) -> anyhow::Result<()> {
    match remote {
        Remote::Azure {
            storage_container: _storage_container,
            storage_account: _storage_account,
        } => todo!(),
    }
}

pub(crate) async fn ledge<T: Storable>(remote: &Remote, _storable: &T) -> anyhow::Result<()> {
    match remote {
        Remote::Azure {
            storage_container: _storage_container,
            storage_account: _storage_account,
        } => todo!(),
    }
}
