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
    #[serde(default)]
    pub acme: Option<String>,
    #[serde(default)]
    pub acmedomain0: Option<String>,
    #[serde(default)]
    pub acmedomain1: Option<String>,
    #[serde(default)]
    pub acmedomain2: Option<String>,
    #[serde(default)]
    pub acmedomain3: Option<String>,
    #[serde(default)]
    pub acmedomain4: Option<String>,
    #[serde(default)]
    pub acmedomain5: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub digest: Option<String>,
    #[serde(default, rename = "startall-onboot-delay")]
    pub start_all_on_boot_delay: Option<String>,
    #[serde(default, rename = "wakeonlan")]
    pub wake_on_lan: Option<String>,
}
