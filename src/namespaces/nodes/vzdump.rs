use std::sync::Arc;

use reqwest::{Client, Url};

use crate::model::node::NodeId;

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
}
