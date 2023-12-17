use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct PveTarget {
    pub name: String, //Name of the target.
    pub origin: String, //Show if this entry was created by a user or was built-in.
    #[serde(rename = "type")]
    pub target_type: TargetType, //Type of the target
    pub comment: Option<String>, //Comment.
    pub disable: Option<bool>, //Disable this matcher.
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    SendMail,
    Gotify
}