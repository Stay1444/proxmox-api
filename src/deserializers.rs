use serde::Deserializer;

use crate::model::node::{NodeId, VMId};

// Proxmox likes to encode empty fields as empty strings
pub fn non_empty_str<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    use serde::Deserialize;
    let o: Option<String> = Option::deserialize(d)?;
    Ok(o.filter(|s| !s.is_empty()))
}

pub fn non_empty_vmid<'de, D: Deserializer<'de>>(d: D) -> Result<Option<VMId>, D::Error> {
    use serde::Deserialize;
    let o: Option<String> = Option::deserialize(d)?;
    Ok(o.filter(|s| !s.is_empty()).map(VMId))
}

pub fn non_empty_nodeid<'de, D: Deserializer<'de>>(d: D) -> Result<Option<NodeId>, D::Error> {
    use serde::Deserialize;
    let o: Option<String> = Option::deserialize(d)?;
    Ok(o.filter(|s| !s.is_empty()).map(NodeId))
}
