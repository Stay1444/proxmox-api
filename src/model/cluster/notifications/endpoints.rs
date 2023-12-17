use serde::Deserialize;
use crate::model::{PveEncryptMode, PveResourceOrigin};

#[derive(Deserialize, Debug)]
pub struct Gotify {
    pub name: String, //The name of the endpoint
    pub origin: Option<PveResourceOrigin>, //Show if this entry was created by user or was built-in
    pub server: String, //Server URL
    pub comment: Option<String>, //Comment
    pub digest: Option<String>, //Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    pub disable: Option<bool> //Disable this target
}

#[derive(Deserialize, Debug)]
pub struct SendMail {
    pub name: String, //The name of the endpoint.
    pub origin: Option<PveResourceOrigin>, //Show if this entry was created by user or was built-in.
    pub author: Option<String>, //Author of the email.
    pub comment: Option<String>, //Comment.
    pub disable: Option<bool>, //Disable this target.
    #[serde(rename = "from-address")]
    pub from_address: Option<String>, //'From' address for the email.
    pub mailto: Option<Vec<String>>, //List of email recipients.
    #[serde(rename = "mailto-user")]
    pub mailto_user: Option<Vec<String>> //List of users.
}

#[derive(Deserialize, Debug)]
pub struct PveSmtp {
    #[serde(rename = "from-address")]
    pub from_address: String, //'From' address for the email.
    pub name: String, //The name of the endpoint.
    pub origin: Option<PveResourceOrigin>, //Show if this entry was created by user or was built-in.
    pub server: String, //The address of the SMTP server.
    pub author: Option<String>, //Author of the mail. Defaults to 'Proxmox VE'.
    pub comment: Option<String>, //Comment
    pub disable: Option<bool>, //Disable this target
    pub mailto: Option<Vec<String>>, //List of recipients
    #[serde(rename = "mailto-user")]
    pub mailto_user: Option<Vec<String>>, //List of users.
    pub mode: Option<PveEncryptMode>, //Determine which encryption method shall be used for the connection.
    pub port: Option<i16>, //The port to be used. Defaults to 465 for TLS based connections, 587 for STARTTLS based connections and port 25 for insecure plain-text connections.
    pub username: Option<String> //Username for SMTP authentication
}