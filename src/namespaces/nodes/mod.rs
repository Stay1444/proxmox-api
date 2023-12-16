use std::{sync::Arc, time::Duration};

use reqwest::{Client, StatusCode, Url};

use crate::{
    error::{ProxmoxAPIError, Result},
    model::{self, node::NodeId, PveResponse, PveVersion},
};

mod tasks;
mod vzdump;

#[derive(Clone)]
pub struct PveNode {
    id: NodeId,
    host: Arc<Url>,
    client: Client,
    pub vzdump: vzdump::VZDump,
    pub tasks: tasks::Tasks,
}

impl PveNode {
    pub(crate) fn new(id: NodeId, host: Arc<Url>, client: Client) -> Self {
        Self {
            id: id.clone(),
            host: host.clone(),
            client: client.clone(),
            vzdump: vzdump::VZDump::new(id.clone(), host.clone(), client.clone()),
            tasks: tasks::Tasks::new(id.clone(), host.clone(), client.clone()),
        }
    }

    pub fn id(&self) -> NodeId {
        self.id.clone()
    }

    /// Try to wake a node via 'wake on LAN' network packet.
    pub async fn wake_on_lan(&self) -> Result<()> {
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
                status => return Err(ProxmoxAPIError::ApiError(status)),
            }
        }

        Ok(())
    }

    /// API version details
    pub async fn version(&self) -> Result<PveVersion> {
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

    pub async fn time(&self) -> Result<model::node::time::Time> {
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
    pub async fn suspend_all(&self) -> Result<()> {
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
                status => return Err(ProxmoxAPIError::ApiError(status)),
            }
        }

        Ok(())
    }

    /// Stop all VMs and Containers.
    pub async fn stop_all(&self, force: bool, timeout: Option<Duration>) -> Result<()> {
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
                status => return Err(ProxmoxAPIError::ApiError(status)),
            }
        }

        Ok(())
    }

    /// Start all VMs and containers located on this node (by default only those with onboot=1).
    pub async fn start_all(&self, force: bool) -> Result<()> {
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
                status => return Err(ProxmoxAPIError::ApiError(status)),
            }
        }

        Ok(())
    }

    /// Gather various systems information about a node
    pub async fn report(&self) -> Result<String> {
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

    /// Query metadata of an URL: file size, file name and mime type.
    pub async fn query_url_metadata(
        &self,
        url: Url,
        verify_certs: bool,
    ) -> Result<model::node::url_metadata::UrlMetadata> {
        let target = url.to_string();

        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/query-url-metadata", self.id))
            .expect("Correct URL");

        let body = serde_json::json!({
            "url": target,
            "verify-certificates": verify_certs
        });

        let response = self
            .client
            .get(url)
            .json(&body)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }

    /// Read tap/vm network device interface counters
    pub async fn netstat(&self) -> Result<model::node::netstat::NetStat> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/netstat", self.id))
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }

    /// Migrate all VMs and Containers.
    pub async fn migrate_all(
        &self,
        target: NodeId,
        with_local_disks: bool,
        max_workers: Option<u32>,
    ) -> Result<()> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/migrateall", self.id))
            .expect("Correct URL");

        let body = serde_json::json!({
            "target": target.to_string(),
            "maxworkers": max_workers,
            "with-local-disks": with_local_disks
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

    /// Get the content of /etc/hosts.
    pub async fn hosts(&self) -> Result<model::node::hosts::Hosts> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/hosts", self.id))
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }

    /// Execute multiple commands in order, root only.
    pub async fn execute(&self, commands: &[model::node::execute::Command]) -> Result<()> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/execute", self.id))
            .expect("Correct URL");

        let commands_as_json =
            serde_json::to_string(&commands).expect("Correct serialization of &[Command]");

        let body = serde_json::json!({
            "commands": commands_as_json
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

    /// Read DNS settings.
    pub async fn dns(&self) -> Result<model::node::dns::DnsSettings> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/dns", self.id))
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }

    /// Get node configuration options.
    pub async fn config(
        &self,
        property: Option<model::node::config::Property>,
    ) -> Result<model::node::config::NodeConfiguration> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/config", self.id))
            .expect("Correct URL");

        let body = serde_json::json!({
            "property": property
        });

        let response = self
            .client
            .get(url)
            .json(&body)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }

    /// Get list of appliances.
    pub async fn apl_info(&self) -> Result<Vec<model::node::aplinfo::ApplianceInformation>> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/aplinfo", self.id))
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }

    /// Read task list for one node (finished tasks).
    ///
    /// You can apply a filter like this:
    /// ```
    /// node.tasks(filter: Some(GetTasksFilter {
    ///     only_errors: Some(true),
    ///     ..Default::default()
    /// }));
    /// ```
    ///
    pub async fn tasks(
        &self,
        filter: Option<model::node::tasks::GetTasksFilter>,
    ) -> Result<Vec<model::node::tasks::Task>> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/tasks", self.id))
            .expect("Correct URL");

        let response = self
            .client
            .get(url)
            .json(&filter)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }
}
