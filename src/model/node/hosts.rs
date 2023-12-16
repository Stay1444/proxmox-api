use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Hosts {
    pub data: String, // The content of /etc/hosts.
    #[serde(default)]
    pub digest: Option<String>, // Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
}
