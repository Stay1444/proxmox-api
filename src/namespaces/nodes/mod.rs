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

    pub fn get(&self, node: impl Into<NodeId>) -> PveNode {
        PveNode {
            id: node.into(),
            host: self.host.clone(),
            client: self.client.clone(),
        }
    }
}

#[derive(Clone)]
pub struct PveNode {
    id: NodeId,
    host: Arc<Url>,
    client: Client,
}

impl PveNode {
    pub fn id(&self) -> NodeId {
        self.id.clone()
    }

    /// Try to wake a node via 'wake on LAN' network packet.
    pub async fn wake_on_lan(&self) -> Result<(), ProxmoxAPIError> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/wakeonlan", self.id))
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
    pub async fn version(&self) -> Result<PveVersion, ProxmoxAPIError> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/version", self.id))
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }

    pub async fn time(&self) -> Result<model::node::Time, ProxmoxAPIError> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/time", self.id))
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
    pub async fn suspend_all(&self) -> Result<(), ProxmoxAPIError> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/suspendall", self.id.clone()))
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
        force: bool,
        timeout: Option<Duration>,
    ) -> Result<(), ProxmoxAPIError> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/stopall", self.id))
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
    pub async fn start_all(&self, force: bool) -> Result<(), ProxmoxAPIError> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/startall", self.id))
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

    /// Gather various systems information about a node
    pub async fn report(&self) -> Result<String, ProxmoxAPIError> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/report", self.id))
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }
}
