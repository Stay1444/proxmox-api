use serde::Deserialize;

use super::VMId;

pub mod clone;
pub mod config;
pub mod interfaces;
pub mod status;

#[derive(Deserialize, Debug)]
pub struct LXC {
    pub status: LXCStatus,
    pub vmid: VMId,

    #[serde(default)]
    pub cpus: Option<f64>,

    #[serde(default)]
    pub lock: Option<String>,

    #[serde(default, rename = "maxdisk")]
    pub max_disk: Option<i64>,
    #[serde(default, rename = "maxmem")]
    pub max_mem: Option<i64>,

    #[serde(default, rename = "maxswap")]
    pub max_swap: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tags: Option<String>,
    #[serde(default)]
    pub uptime: Option<i64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LXCStatus {
    Stopped,
    Running,
}
