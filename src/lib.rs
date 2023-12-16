use std::sync::Arc;

use auth::ProxmoxAuthentication;
use error::{ProxmoxAPIError, Result};
use model::node::NodeId;
use namespaces::{cluster::PveCluster, nodes::PveNode};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Url,
};

use crate::model::PveResponse;

pub mod auth;
pub mod error;
pub mod model;
mod namespaces;

#[derive(Clone)]
pub struct ProxmoxClient {
    host: Arc<Url>,
    client: reqwest::Client,

    pub cluster: PveCluster,
}

impl ProxmoxClient {
    pub fn new(host: Url, auth: ProxmoxAuthentication) -> Self {
        let mut headers = HeaderMap::new();
        headers.append(
            "Authorization",
            HeaderValue::from_str(&format!(
                "PVEAPIToken={}@{}!{}={}",
                auth.user, auth.realm, auth.token.name, auth.token.value
            ))
            .expect("Valid HeaderValue"),
        );

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .default_headers(headers)
            .build()
            .expect("Valid reqwest client");

        let host = Arc::new(host);

        Self {
            host: host.clone(),
            client: client.clone(),
            cluster: PveCluster::new(host.clone(), client.clone()),
        }
    }

    pub async fn version(&self) -> Result<model::PveVersion> {
        let url = self.host.join("/api2/json/version").expect("Correct URL");
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }

    pub fn node(&self, id: impl Into<NodeId>) -> PveNode {
        PveNode::new(id.into(), self.host.clone(), self.client.clone())
    }

    /// Cluster node index.
    pub async fn nodes(&self) -> Result<Vec<model::node::PveNodeInformation>> {
        let url = self.host.join("/api2/json/nodes").expect("Correct URL");
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        Ok(PveResponse::from_response(response).await?.data)
    }
}
