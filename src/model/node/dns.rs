use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DnsSettings {
    pub dns1: String,   // First name server IP address.
    pub dns2: String,   // Second name server IP address.
    pub dns3: String,   // Thid name server IP address.
    pub search: String, // Search domain for host-name lookup.
}
