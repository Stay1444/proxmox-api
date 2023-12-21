use std::sync::Arc;
use std::time::Duration;

use reqwest::{Client, StatusCode, Url};

use crate::error::{ProxmoxAPIError, Result};
use crate::model::node::{NodeId, VMId};
use crate::model::{self, PveResponse};

#[derive(Clone)]
pub struct PveLXCStatus {
    node_id: NodeId,
    id: VMId,
    host: Arc<Url>,
    client: Client,
}

impl PveLXCStatus {
    pub(crate) fn new(node_id: NodeId, id: VMId, host: Arc<Url>, client: Client) -> Self {
        Self {
            id: id.clone(),
            node_id: node_id.clone(),
            host: host.clone(),
            client: client.clone(),
        }
    }

    pub fn id(&self) -> VMId {
        self.id.clone()
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id.clone()
    }

    /// Suspend the container. This is experimental.
    pub async fn suspend(&self) -> Result<()> {
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/status/suspend",
                self.node_id, self.id
            ))
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
                status => return Err(ProxmoxAPIError::ApiError(status)),
            }
        }

        Ok(())
    }

    /// Stop the container. This will abruptly stop all processes running in the container.
    /// * `skip_lock` - Ignore locks - only root is allowed to use this option.
    pub async fn stop(&self, skip_lock: bool) -> Result<()> {
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/status/stop",
                self.node_id, self.id
            ))
            .expect("Correct URL");

        let body = serde_json::json!({
            "skiplock": skip_lock
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
                status => return Err(ProxmoxAPIError::ApiError(status)),
            }
        }

        Ok(())
    }

    /// Start the container.
    /// * `skip_lock` - Ignore locks - only root is allowed to use this option.
    pub async fn start(&self, skip_lock: bool, debug: bool) -> Result<()> {
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/status/start",
                self.node_id, self.id
            ))
            .expect("Correct URL");

        let body = serde_json::json!({
            "skiplock": skip_lock,
            "debug": debug
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
                status => return Err(ProxmoxAPIError::ApiError(status)),
            }
        }

        Ok(())
    }

    /// Shutdown the container. This will trigger a clean shutdown of the container, see lxc-stop(1) for details.
    pub async fn shutdown(&self, force: bool, timeout: Option<Duration>) -> Result<()> {
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/status/shutdown",
                self.node_id, self.id
            ))
            .expect("Correct URL");

        let body = serde_json::json!({
            "forceStop": force,
            "timeout": timeout.map(|x| x.as_secs())
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
                status => return Err(ProxmoxAPIError::ApiError(status)),
            }
        }

        Ok(())
    }

    /// Resume the container.
    pub async fn resume(&self) -> Result<()> {
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/status/resume",
                self.node_id, self.id
            ))
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
                status => return Err(ProxmoxAPIError::ApiError(status)),
            }
        }

        Ok(())
    }

    /// Reboot the container by shutting it down, and starting it again. Applies pending changes.
    pub async fn reboot(&self, timeout: Option<Duration>) -> Result<()> {
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/status/reboot",
                self.node_id, self.id
            ))
            .expect("Correct URL");

        let body = serde_json::json!({
            "timeout": timeout.map(|x| x.as_secs())
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
                status => return Err(ProxmoxAPIError::ApiError(status)),
            }
        }

        Ok(())
    }

    pub async fn status(&self) -> Result<model::node::lxc::LXC> {
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/status/current",
                self.node_id, self.id
            ))
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
