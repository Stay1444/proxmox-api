use serde::{Deserialize, Serialize};
use serde::de::Unexpected::Option;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SdnControllerType {
    BGP,
    eVPN,
    Faucet,
    Isis
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SdnZoneType {
    eVPN,
    Faucet,
    QINQ,
    Simple,
    Vlan,
    Vxlan
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SdnIpamType {
    Netbox,
    Phpipam,
    Pve
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SdnController {
    pub controller: String, //The SDN controller object identifier.
    #[serde(rename = "type")]
    pub controller_type: String,
    pub pending: Option<String>, //Display pending config
    pub state: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SdnDNS {
    pub dns: String,
    #[serde(rename = "type")]
    pub dns_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SdnIpam {
    pub ipam: String,
    #[serde(rename = "type")]
    pub ipam_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SdnZone {
    #[serde(rename = "type")]
    pub zone_type: SdnZoneType,
    pub zone: String, //The SDN zone object identifier.
    pub dhcp: Option<String>,
    pub dns: Option<String>,
    pub dnszone: Option<String>,
    pub ipam: Option<String>,
    pub mtu: Option<i16>,
    pub nodes: Option<String>,
    pub pending: Option<String>, //Display pending config
    pub reversedns: Option<String>,
    pub state: Option<String>
}



