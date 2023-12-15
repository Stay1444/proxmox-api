use std::{sync::Arc, time::Duration};

use reqwest::{Client, StatusCode, Url};

use crate::{
    error::ProxmoxAPIError,
    model::{self, node::NodeId, PveResponse, PveVersion},
};

#[derive(Clone)]
pub struct PveNodes {
    host: Arc<Url>,
    client: Client,
}

impl PveNodes {
    pub fn new(host: Arc<Url>, client: Client) -> Self {
        Self { client, host }
    }

    /// Try to wake a node via 'wake on LAN' network packet.
    pub async fn wake_on_lan(&self, node: impl Into<NodeId>) -> Result<(), ProxmoxAPIError> {
        let node = node.into();
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{node}/wakeonlan"))
            .expect("Correct URL");

        let response = self
            .client
            .post(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        if !response.status().is_success() {
            match response.status() {
                StatusCode::UNAUTHORIZED => return Err(ProxmoxAPIError::Unauthorized),
                _ => return Err(ProxmoxAPIError::ApiError),
            }
        }

        Ok(())
    }

    /// API version details
    pub async fn version(&self, node: impl Into<NodeId>) -> Result<PveVersion, ProxmoxAPIError> {
        let node = node.into();
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{node}/version"))
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }

    pub async fn time(
        &self,
        node: impl Into<NodeId>,
    ) -> Result<model::node::Time, ProxmoxAPIError> {
        let node = node.into();
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{node}/time"))
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }

    /// Suspend all VMs.
    pub async fn suspend_all(&self, node: impl Into<NodeId>) -> Result<(), ProxmoxAPIError> {
        let node = node.into();

        let url = self
            .host
            .join(&format!("/api2/json/nodes/{node}/suspendall"))
            .expect("Correct URL");

        let response = self
            .client
            .post(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        if !response.status().is_success() {
            match response.status() {
                StatusCode::UNAUTHORIZED => return Err(ProxmoxAPIError::Unauthorized),
                _ => return Err(ProxmoxAPIError::ApiError),
            }
        }

        Ok(())
    }

    /// Stop all VMs and Containers.
    pub async fn stop_all(
        &self,
        node: impl Into<NodeId>,
        force: bool,
        timeout: Option<Duration>,
    ) -> Result<(), ProxmoxAPIError> {
        let node = node.into();

        let url = self
            .host
            .join(&format!("/api2/json/nodes/{node}/stopall"))
            .expect("Correct URL");

        let body = serde_json::json!({
            "force-stop": force,
            "timeout": timeout.map(|duration| duration.as_secs())
        });

        let response = self
            .client
            .post(url)
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

        Ok(())
    }

    /// Start all VMs and containers located on this node (by default only those with onboot=1).
    pub async fn start_all(
        &self,
        node: impl Into<NodeId>,
        force: bool,
    ) -> Result<(), ProxmoxAPIError> {
        let node = node.into();

        let url = self
            .host
            .join(&format!("/api2/json/nodes/{node}/startall"))
            .expect("Correct URL");

        let body = serde_json::json!({
            "force": force,
        });

        let response = self
            .client
            .post(url)
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

        Ok(())
    }
}
