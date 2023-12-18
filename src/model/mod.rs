use reqwest::{Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::error;

use crate::error::ProxmoxAPIError;

pub mod cluster;
pub mod node;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PveResponse<T> {
    pub data: T,
}

impl<T> PveResponse<T>
where
    T: DeserializeOwned,
{
    pub(crate) async fn from_response(
        response: Response,
    ) -> Result<PveResponse<T>, ProxmoxAPIError> {
        if !response.status().is_success() {
            match response.status() {
                StatusCode::UNAUTHORIZED => return Err(ProxmoxAPIError::Unauthorized),
                status => return Err(ProxmoxAPIError::ApiError(status)),
            }
        }

        let body = response
            .text()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        dbg!(&body);

        serde_json::from_str(&body).map_err(|err| {
            error!("{err}");
            ProxmoxAPIError::DeserializationError
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PveVersion {
    pub release: String, // The current Proxmox VE point release in `x.y` format.
    pub repoid: String,  // The short git revision from which this version was build.
    pub version: String, // The full pve-manager package version of this node.
    #[serde(default)]
    pub console: Option<PveConsoleViewer>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PveConsoleViewer {
    Applet,
    VV,
    Html5,
    XTermJS,
}

pub enum Size {
    TB(i64),
    GB(i64),
    MB(i64),
    KB(i64),
    B(i64),
}
