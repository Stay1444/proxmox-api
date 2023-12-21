use std::sync::Arc;
use std::time::Duration;

use reqwest::{Client, StatusCode, Url};

use crate::error::{ProxmoxAPIError, Result};
use crate::model::node::{NodeId, VMId};
use crate::model::{self, PveResponse, Size};

#[derive(Clone)]
pub struct PveLXC {
    node_id: NodeId,
    id: VMId,
    host: Arc<Url>,
    client: Client,
}

impl PveLXC {
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

    /// Create a Template.
    pub async fn template(&self) -> Result<()> {
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/template",
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

    /// Resize a container mount point. Shrinking not supported.
    /// * `size` - The target size
    /// * `additive` - Add the size to the actual size
    /// * `disk` - The disk you want to resize.
    /// * `digest` - Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    /// ```
    /// // Adds 256 GB
    /// lxc.resize(Size::GB(256.0), true, "rootfs", None);
    /// ```
    pub async fn resize(
        &self,
        size: model::Size,
        additive: bool,
        disk: impl Into<&str>,
        digest: Option<&str>,
    ) -> Result<()> {
        let disk = disk.into();

        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/resize",
                self.node_id, self.id
            ))
            .expect("Correct URL");

        let size = if !additive {
            size.to_string()
        } else {
            format!("+{}", size.to_string())
        };

        let body = serde_json::json!({
            "disk": disk,
            "size": size.to_string(),
            "digest": digest
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

    /// Migrate the container to another node. Creates a new migration task.
    /// * `target` - The cluster node name.
    /// * `bandwidth_limit` - Override I/O bandwidth limit.
    /// * `online` - Use online/live migration.
    /// * `restart` - Use restart migration
    /// * `target_storage` - Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself.
    /// * `timeout` - Timeout in seconds for shutdown for restart migration
    ///
    /// Example:
    /// ```
    /// lxc.migrate("pve01", None, true, false, None, None)
    /// ```
    pub async fn migrate(
        &self,
        target: NodeId,
        bandwidth_limit: Option<Size>,
        online: bool,
        restart: bool,
        target_storage: Option<&str>,
        timeout: Option<Duration>,
    ) -> Result<()> {
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/migrate",
                self.node_id, self.id
            ))
            .expect("Correct URL");

        let body = serde_json::json!({
            "target": target.0,
            "bwlimit": bandwidth_limit.map(|x| x.to_kb()),
            "online": online,
            "restart": restart,
            "target-storage": target_storage,
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

    /// Get IP addresses of the specified container interface.
    pub async fn interfaces(&self) -> Result<Vec<model::node::lxc::interfaces::Interface>> {
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/interfaces",
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

    /// Get container configuration.
    /// * `current` - Get current values (instead of pending values).
    /// * `snapshot` - Fetch config values from given snapshot.
    /// ```
    /// lxc.config(true, None);
    pub async fn config(
        &self,
        current: bool,
        snapshot: Option<&str>,
    ) -> Result<model::node::lxc::config::LXCConfiguration> {
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/config",
                self.node_id, self.id
            ))
            .expect("Correct URL");

        let body = serde_json::json!({
            "current": current,
            "snapshot": snapshot
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

    /// Create a container clone/copy
    /// ```
    /// lxc.clone(Parameters {
    ///     new_id: VMId("200".into()),
    ///     ..Default::default()
    /// })
    /// ```
    pub async fn clone<'a>(
        &self,
        parameters: model::node::lxc::clone::Parameters<'a>,
    ) -> Result<()> {
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc/{}/clone",
                self.node_id, self.id
            ))
            .expect("Correct URL");

        let body = serde_json::json!({
            "newid": parameters.new_id,
            "bwlimit": parameters.bandwidth_limit.map(|x| x.to_kb()),
            "description": parameters.description,
            "full": parameters.full,
            "hostname": parameters.hostname,
            "pool": parameters.pool,
            "snapname": parameters.snapname,
            "storage": parameters.storage,
            "target": parameters.target
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
}
