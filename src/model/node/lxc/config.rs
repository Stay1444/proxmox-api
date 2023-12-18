use serde::Deserialize;

use crate::model::{Architecture, OSType};

#[derive(Deserialize, Debug)]
pub struct LXCConfiguration {
    pub digest: String,

    #[serde(default)]
    pub arch: Option<Architecture>,
    #[serde(rename = "cmode", default)]
    pub console_mode: Option<ConsoleMode>,
    #[serde(default)]
    pub console: Option<bool>,
    #[serde(default)]
    pub cores: Option<u64>,
    #[serde(default, rename = "cpulimit")]
    pub cpu_limit: Option<f64>,
    #[serde(default, rename = "cpuunits")]
    pub cpu_units: Option<u64>,
    #[serde(default)]
    pub debug: Option<bool>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub features: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub hookscript: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub hostname: Option<String>,
    #[serde(default)]
    pub lxc: Vec<Vec<String>>,
    #[serde(default)]
    pub memory: Option<u64>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub nameserver: Option<String>,
    #[serde(default)]
    pub onboot: Option<bool>,
    #[serde(default)]
    pub ostype: Option<OSType>,
    #[serde(default)]
    pub protection: Option<bool>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub searchdomain: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub startup: Option<String>,
    #[serde(default)]
    pub swap: Option<u64>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub tags: Option<String>,
    #[serde(default)]
    pub template: Option<bool>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub timezone: Option<String>,
    #[serde(default)]
    pub tty: Option<u64>,
    #[serde(default)]
    pub unprivileged: Option<bool>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConsoleMode {
    Shell,
    Console,
    TTY,
}
