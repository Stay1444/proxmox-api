use std::fmt::format;
use std::sync::Arc;
use reqwest::{Client, StatusCode, Url};
use crate::error::ProxmoxAPIError;
use crate::model::cluster::PveClusterStatus;
use crate::model::{PveResourceType, PveResponse};

#[derive(Clone)]
pub struct PveCluster {
    host: Arc<Url>,
    client: Client,
}

impl PveCluster {

    pub fn new(host: Arc<Url>, client: Client) -> Self {
        Self {
            client,
            host
        }
    }

    pub async fn tasks(&self) -> Result<Vec<String>, ProxmoxAPIError> {
        let url = self.
            host
            .join("/api2/json/cluster/tasks")
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        if !response.status().is_success() {
            match response.status() {
                StatusCode::UNAUTHORIZED => return Err(ProxmoxAPIError::Unauthorized),
                _ => return Err(ProxmoxAPIError::ApiError),
            }
        }

        Ok(PveResponse::from_response(response).await?.data)
    }

    pub async fn status(&self) -> Result<PveClusterStatus, ProxmoxAPIError> {
        let url = self
            .host
            .join("/api2/json/cluster/status")
            .expect("Correct URL");

        let response = self.
            client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        if !response.status().is_success() {
            match response.status() {
                StatusCode::UNAUTHORIZED => return Err(ProxmoxAPIError::Unauthorized),
                _ => return Err(ProxmoxAPIError::ApiError),
            }
        }


        Ok(PveResponse::from_response(response).await?.data)
    }

    pub async fn resources(&self, resource: PveResourceType) -> Result<(), ProxmoxAPIError> {
        let url = self
            .host
            .join("/api2/json/cluster/resources")
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .form(&("type", resource))
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;


    }

}