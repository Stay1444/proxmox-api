use serde::{Deserialize, Serialize};

use crate::model::PveType;

#[derive(Serialize, Deserialize, Debug)]
pub struct PveClusterStatus {
    pub id: String,
    pub ip: Option<String>,    // [node] IP of the resolved nodename.
    pub level: Option<String>, // [node] Proxmox VE Subscription level, indicates if eligible for enterprise support as well as access to the stable Proxmox VE Enterprise Repository.
    pub local: Option<bool>,   // [node] Indicates if this is the responding node.
    pub name: String,
    pub nodeid: Option<i32>, // [node] ID of the node from the corosync configuration.
    pub nodes: Option<i32>,  // [cluster] Nodes count, including offline nodes.
    pub online: Option<bool>, // [node] Indicates if the node is online or offline.
    pub quorate: Option<bool>, // [cluster] Indicates if there is a majority of nodes online to make decisions
    #[serde(rename = "type")]
    pub pve_type: PveType, // Indicates the type, either cluster or node. The type defines the object properties e.g. quorate available for type cluster.
    pub version: Option<i32>, // [cluster] Current version of the corosync configuration file.
}
