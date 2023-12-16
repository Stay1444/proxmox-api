use crate::error::ProxmoxAPIError;
use crate::model::cluster::resources::PveClusterResource;
use crate::model::cluster::status::PveClusterStatus;
use crate::model::{PveResourceType, PveResponse};
use reqwest::{Client, StatusCode, Url};
use serde_json::Value;
use std::sync::Arc;

pub mod sdn;

#[derive(Clone)]
pub struct PveCluster {
    host: Arc<Url>,
    client: Client,
}

impl PveCluster {
    pub fn new(host: Arc<Url>, client: Client) -> Self {
        Self { client, host }
    }

    pub async fn tasks(&self) -> Result<Vec<String>, ProxmoxAPIError> {
        let url = self
            .host
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

    pub async fn status(&self) -> Result<Vec<PveClusterStatus>, ProxmoxAPIError> {
        let url = self
            .host
            .join("/api2/json/cluster/status")
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

    pub async fn resources(
        &self,
        resource: PveResourceType,
    ) -> Result<Vec<PveClusterResource>, ProxmoxAPIError> {
        let url = self
            .host
            .join("/api2/json/cluster/resources")
            .expect("Correct URL");

        let body = serde_json::json!({
            "type": resource,
        });

        let response = self
            .client
            .get(url)
            .json(&body)
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

    pub async fn options(&self) -> Result<Value, ProxmoxAPIError> {
        let url = self
            .host
            .join("/api2/json/cluster/options")
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        if !response.status().is_success() {
            return match response.status() {
                StatusCode::UNAUTHORIZED => Err(ProxmoxAPIError::Unauthorized),
                _ => Err(ProxmoxAPIError::ApiError),
            };
        }

        Ok(PveResponse::from_response(response).await?.data)
    }

    pub async fn next_id(&self) -> Result<i32, ProxmoxAPIError> {
        let url = self
            .host
            .join("/api2/json/cluster/nextid")
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        if !response.status().is_success() {
            return match response.status() {
                StatusCode::UNAUTHORIZED => Err(ProxmoxAPIError::Unauthorized),
                _ => Err(ProxmoxAPIError::ApiError),
            };
        }

        Ok(PveResponse::from_response(response).await?.data)
    }

    pub async fn log(&self) -> Result<Vec<Value>, ProxmoxAPIError> {
        let url = self
            .host
            .join("/api2/json/cluster/log")
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        if !response.status().is_success() {
            return match response.status() {
                StatusCode::UNAUTHORIZED => Err(ProxmoxAPIError::Unauthorized),
                _ => Err(ProxmoxAPIError::ApiError),
            };
        }

        Ok(PveResponse::from_response(response).await?.data)
    }
}
