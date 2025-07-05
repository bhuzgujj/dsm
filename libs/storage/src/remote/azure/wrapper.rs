use std::process::Command;

use azure_storage::StorageCredentials;
use azure_storage_datalake::prelude::DataLakeClient;
use futures::StreamExt;
use interfaces::log_err;
use serde::de::DeserializeOwned;

use crate::storable::Storable;

use super::{access_token::AccessTokenFromCli, assets::info::DataAssets};

pub struct AzureClientWrapper {
    storage_account: String,
    access_token: Option<AccessTokenFromCli>,
}

impl AzureClientWrapper {
    pub(crate) fn new(storage_account: String) -> Self {
        Self {
            storage_account,
            access_token: Command::new("cmd")
                .args(["/C", "az account get-access-token"])
                .output()
                .map_or(None, |output|  String::from_utf8(output.stdout)
                    .map_or(None, |json_str| serde_json::from_str(&json_str)
                        .map_or(None, |token: AccessTokenFromCli| Some(token))))
        }
    }

    fn token_is_expired(&self) -> bool {
        self.access_token.as_ref().is_none_or(AccessTokenFromCli::is_expired)
    }

    fn query<T: DeserializeOwned>(&self, query: &str) -> anyhow::Result<T> {
        let json_str = String::from_utf8(
            Command::new("cmd")
                .args(["/C", query])
                .output()?
                .stdout,
        )?;
        let result: T = serde_json::from_str(&json_str)?;
        Ok(result)
    }

    fn exec(&self, query: &str) -> anyhow::Result<()> {
        Command::new("cmd")
            .args(["/C", query])
            .output()?;
        Ok(())
    }

    pub(crate) async fn login(&mut self) -> anyhow::Result<()> {
        self.exec("az login")?;
        self.access_token = Some(self.query("az account get-access-token")?);
        Ok(())
    }

    pub(crate) async fn list_data_assets<T: Storable>(&mut self) -> anyhow::Result<Vec<T>> {
        if self.token_is_expired() {
            self.login().await?;
        }

        let result: Vec<DataAssets> = self.query("az ml data list")?;
        Ok(result.iter()
            .map(DataAssets::to_dsm)
            .map(Storable::from_dsm)
            .collect())
    }

    pub(crate) async fn _get_data_assets(&mut self) -> anyhow::Result<()> {
        if self.token_is_expired() {
            self.login().await?;
        }

        let result: Vec<DataAssets> = self.query("az ml data list")?;
        dbg!(result);
        Ok(())
    }

    pub(crate) async fn _get_blob(&mut self, storage_container: String) -> anyhow::Result<()> {
        if self.token_is_expired() {
            self.login().await?;
        }
        let bearer_token = match &self.access_token {
            Some(at) => at.accessToken.clone(),
            None => return log_err!(format!("Could not get an access token")),
        };
        let store_credentials = StorageCredentials::bearer_token(bearer_token);
        let client = DataLakeClient::new(self.storage_account.clone(), store_credentials);
        let file_system_client = client.file_system_client(storage_container);
        //let client = ClientBuilder::new(self.storage_account, store_credentials.clone()).container_client(storage_container);
        
        let mut result = file_system_client.list_paths().into_stream();

        while let Some(path_list) = result.next().await {
            match path_list {
                Ok(path_resp) => { 
                    for p in path_resp.paths {
                        println!("P: {}", p.name);
                    }
                },
                Err(err) => return log_err!(format!("Err: {}", err)),
            }
        }

        Ok(())
    }
}