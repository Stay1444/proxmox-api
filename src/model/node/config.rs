use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub enum Property {
    ACME,
    ACMEDomain0,
    ACMEDomain1,
    ACMEDomain2,
    ACMEDomain3,
    ACMEDomain4,
    ACMEDomain5,
    Description,
    StartAllOnBootDelay,
    WakeOnLan,
}

#[derive(Deserialize, Debug)]
pub struct NodeConfiguration {
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acme: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acmedomain0: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acmedomain1: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acmedomain2: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acmedomain3: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acmedomain4: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acmedomain5: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub digest: Option<String>,
    #[serde(
        default,
        rename = "startall-onboot-delay",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub start_all_on_boot_delay: Option<String>,
    #[serde(
        default,
        rename = "wakeonlan",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub wake_on_lan: Option<String>,
}
