use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PveClusterResource {
    pub id: String, // Resource id.
    #[serde(rename = "type")]
    pub resource_type: ResourceType, // Resource type.
    #[serde(rename = "cgroup-mode")]
    pub cgroup_mode: Option<i32>, // The cgroup mode the node operates under (when type == node).
    pub content: Option<String>, // Allowed storage content types (when type == storage).
    pub cpu: Option<u16>, // CPU utilization (when type in node,qemu,lxc).
    pub disk: Option<i64>, // Used disk space in bytes (when type in storage), used root image spave for VMs (type in qemu,lxc).
    pub hastate: Option<u16>, // HA service status (for HA managed VMs).
    pub level: Option<String>, // Support level (when type == node).
    pub maxcpu: Option<u16>, // Number of available CPUs (when type in node,qemu,lxc).
    pub maxdisk: Option<i64>, // Storage size in bytes (when type in storage), root image size for VMs (type in qemu,lxc).
    pub maxmem: Option<i32>,  // Number of available memory in bytes (when type in node,qemu,lxc).
    pub mem: Option<i32>,     // Used memory in bytes (when type in node,qemu,lxc).
    pub name: Option<String>, // Name of the resource.
    pub node: Option<String>, // The cluster node name (when type in node,storage,qemu,lxc).
    #[serde(rename = "plugintype")]
    pub plugin_type: Option<String>, // More specific type, if available.
    pub pool: Option<String>, // The pool name (when type in pool,qemu,lxc).
    pub status: Option<String>, // Resource type dependent status.
    pub storage: Option<String>, // The storage identifier (when type == storage).
    pub uptime: Option<i64>,  // Node uptime in seconds (when type in node,qemu,lxc).
    pub vmid: Option<i16>,    // The numerical vmid (when type in qemu,lxc).
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
