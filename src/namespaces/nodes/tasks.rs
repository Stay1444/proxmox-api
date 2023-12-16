use std::sync::Arc;

use reqwest::{Client, Url};

use crate::model::node::NodeId;
use crate::model::{self, PveResponse};

use crate::error::{ProxmoxAPIError, Result};

#[derive(Clone)]
pub struct Tasks {
    id: NodeId,
    host: Arc<Url>,
    client: Client,
}

impl Tasks {
    pub(crate) fn new(id: NodeId, host: Arc<Url>, client: Client) -> Self {
        Self { id, host, client }
    }

    pub fn id(&self) -> NodeId {
        self.id.clone()
    }

    /// Read task status.
    pub async fn status(&self, upid: impl Into<String>) -> Result<model::node::tasks::TaskStatus> {
        let upid = upid.into();
        let url = self
            .host
            .join(&format!(
                "/api2/json/nodes/{}/tasks/{}/status",
                self.id, upid
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
