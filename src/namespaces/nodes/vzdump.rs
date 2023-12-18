use std::sync::Arc;

use reqwest::{Client, Url};

use crate::model::node::NodeId;
use crate::model::{self, PveResponse};

use crate::error::{ProxmoxAPIError, Result};

#[derive(Clone)]
pub struct VZDump {
    id: NodeId,
    host: Arc<Url>,
    client: Client,
}

impl VZDump {
    pub(crate) fn new(id: NodeId, host: Arc<Url>, client: Client) -> Self {
        Self { id, host, client }
    }

    pub fn id(&self) -> NodeId {
        self.id.clone()
    }

    /// API version details
    pub async fn defaults(
        &self,
        storage_identifier: Option<&str>,
    ) -> Result<model::node::vzdump::defaults::ConfiguredDefaults> {
        let url = self
            .host
            .join(&format!("/api2/json/nodes/{}/vzdump/defaults", self.id))
            .expect("Correct URL");

        let body = serde_json::json!({
            "storage": storage_identifier
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
}
