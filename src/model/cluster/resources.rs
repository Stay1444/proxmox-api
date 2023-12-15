use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PveClusterResource {
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
    pub resource_type: ResourceType,
    pub uptime: Option<i64>,
    pub vmid: Option<i16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceType {
    Node,
    Storage,
    Pool,
    Qemu,
    LXC,
    OpenVZ,
    SDN,
}
