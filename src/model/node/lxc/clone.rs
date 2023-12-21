use crate::model::{
    node::{NodeId, VMId},
    Size,
};

#[derive(Debug)]
pub struct Parameters<'a> {
    pub new_id: VMId,                  // VMID for the clone.
    pub bandwidth_limit: Option<Size>, // Override I/O bandwidth limit.
    pub description: Option<&'a str>,  // Description for the new CT.
    pub full: Option<bool>, // Create a full copy of all disks. This is always done when you clone a normal CT. For CT templates, we try to create a linked clone by default.
    pub hostname: Option<&'a str>, // Set a hostname for the new CT.
    pub pool: Option<&'a str>, // Add the new CT to the specified pool.
    pub snapname: Option<&'a str>, // The name of the snapshot.
    pub storage: Option<&'a str>, // Target storage for full clone.
    pub target: Option<NodeId>, // Target node. Only allowed if the original VM is on shared storage.
}

impl<'a> Default for Parameters<'a> {
    fn default() -> Self {
        Self {
            new_id: VMId("100".into()),
            bandwidth_limit: Default::default(),
            description: Default::default(),
            full: Default::default(),
            hostname: Default::default(),
            pool: Default::default(),
            snapname: Default::default(),
            storage: Default::default(),
            target: Default::default(),
        }
    }
}
