use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApplianceInformation {
    pub template: String,
    pub headline: String,
    pub source: String,
    pub description: String,
    pub location: String,
    #[serde(rename = "type")]
    pub appliance_type: String,
    pub infopage: String,
    pub os: String,
    pub version: String,
    #[serde(rename = "manageurl")]
    pub manage_url: String,
    pub section: String,
    pub sha512sum: String,
    pub architecture: String,
    pub package: String,
}
