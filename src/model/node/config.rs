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
    pub acme: Option<String>, // Node specific ACME settings.
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acmedomain0: Option<String>, // ACME domain and validation plugin
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acmedomain1: Option<String>, // ACME domain and validation plugin
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acmedomain2: Option<String>, // ACME domain and validation plugin
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acmedomain3: Option<String>, // ACME domain and validation plugin
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acmedomain4: Option<String>, // ACME domain and validation plugin
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub acmedomain5: Option<String>, // ACME domain and validation plugin
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub description: Option<String>, // Description for the Node. Shown in the web-interface node notes panel. This is saved as comment inside the configuration file.
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub digest: Option<String>, // Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    #[serde(default, rename = "startall-onboot-delay")]
    pub start_all_on_boot_delay: Option<u64>, // Initial delay in seconds, before starting all the Virtual Guests with on-boot enabled.
    #[serde(
        default,
        rename = "wakeonlan",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub wake_on_lan: Option<String>, // MAC address for wake on LAN
}
