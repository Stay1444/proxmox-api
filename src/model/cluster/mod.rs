use serde::{Deserialize, Serialize};

mod notifications;
pub mod resources;
pub mod sdn;
pub mod status;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PveResourceType {
    VM,
    Storage,
    Node,
    Sdn,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PveResourceOrigin {
    #[serde(rename = "user-created")]
    UserCreated,
    Builtin,
    #[serde(rename = "modified-builtin")]
    ModifiedBuiltIn,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PveEncryptMode {
    Insecure,
    StartTls,
    Tls,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PveType {
    Cluster,
    Node,
}
