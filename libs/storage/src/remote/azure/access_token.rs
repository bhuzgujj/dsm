use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub(crate) struct AccessTokenFromCli {
    pub(crate) accessToken: String,
    pub(crate) expiresOn: String,
    pub(crate) expires_on: u64,
    pub(crate) subscription: String,
    pub(crate) tenant: String,
    pub(crate) tokenType: String,
}

impl AccessTokenFromCli {
    pub fn is_expired(&self) -> bool {
        chrono::Local::now().timestamp() as u64 > self.expires_on
    }
}