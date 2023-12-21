use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub mod aplinfo;
pub mod config;
pub mod dns;
pub mod execute;
pub mod hosts;
pub mod netstat;
pub mod time;
pub mod url_metadata;

pub mod lxc;
pub mod tasks;
pub mod vzdump;

#[derive(Deserialize, Debug)]
pub struct PveNodeInformation {
    #[serde(rename = "node")]
    pub id: NodeId,
    pub status: NodeStatus,
    #[serde(default)]
    pub cpu: Option<f64>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub level: Option<String>,
    #[serde(default, rename = "maxcpu")]
    pub max_cpu: Option<i64>,
    #[serde(default, rename = "maxmem")]
    pub max_mem: Option<i64>,
    #[serde(default)]
    pub mem: Option<i64>,
    #[serde(default)]
    pub ssl_fingerprint: Option<String>,
    #[serde(default)]
    pub uptime: Option<i64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NodeStatus {
    Unknown,
    Online,
    Offline,
}

impl Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Online => write!(f, "Online"),
            Self::Offline => write!(f, "Offline"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NodeId(pub(crate) String);

impl NodeId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for NodeId {
    fn from(val: &str) -> Self {
        NodeId(val.into())
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl From<String> for NodeId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VMId(pub(crate) String);

impl VMId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for VMId {
    fn from(val: &str) -> Self {
        VMId(val.into())
    }
}

impl std::fmt::Display for VMId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl From<String> for VMId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
