use serde::{Deserialize, Serialize};
use crate::model::{AdvancedPveResourceType, PveResourceType, PveType};

#[derive(Serialize, Deserialize, Debug)]
pub struct PveClusterStatus {
    pub id: String,
    pub ip: Option<String>,
    pub level: Option<String>,
    pub local: Option<bool>,
    pub name: String,
    pub nodeid: Option<i32>,
    pub nodes: Option<i32>,
    pub online: Option<bool>,
    pub quorate: Option<bool>,
    #[serde(rename = "type")]
    pub pvetype: PveType,
    pub version: Option<i32>

}

#[derive(Serialize, Deserialize, Debug)]
pub struct PveClusterResources {
    pub id: String,
    #[serde(rename = "cgroup-mode")]
    pub cgroup_mode: Option<i32>,
    pub content: Option<String>,
    pub cpu: Option<u16>,
    pub disk: Option<i64>,
    pub hastate: Option<u16>,
    pub level: Option<String>,
    pub maxcpu: Option<u16>,
    pub maxdisk: Option<i64>,
    pub maxmem: Option<i32>,
    pub mem: Option<i32>,
    pub name: Option<String>,
    pub node: Option<String>,
    pub plugintype: Option<String>,
    pub pool: Option<String>,
    pub status: Option<String>, //Why?
    pub storage: Option<String>,
    #[serde(rename = "type")]
    pub resourcetype: AdvancedPveResourceType,
    pub uptime: Option<i64>,
    pub vmid: Option<i16>

}