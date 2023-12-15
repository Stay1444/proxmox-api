use serde::{Deserialize, Serialize};

use crate::model::PveType;

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
    pub pve_type: PveType,
    pub version: Option<i32>,
}
