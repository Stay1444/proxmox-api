use std::sync::Arc;

use reqwest::{Client, StatusCode, Url};

use crate::error::{ProxmoxAPIError, Result};
use crate::model;
use crate::model::node::{NodeId, VMId};

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

    pub async fn resize(
        &self,
        size: model::Size,
        disk: impl Into<String>,
        digest: Option<impl Into<String>>,
    ) -> Result<()> {
        let disk = disk.into();
        let digest = digest.map(|x| x.into());

        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/lxc{}/resize",
                self.node_id, self.id
            ))
            .expect("Correct URL");

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
}
