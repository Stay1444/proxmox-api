use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NetStat {
    pub vmid: String,
    #[serde(rename = "in")]
    pub traffic_in: String,
    #[serde(rename = "out")]
    pub traffic_out: String,
    pub dev: String,
}
