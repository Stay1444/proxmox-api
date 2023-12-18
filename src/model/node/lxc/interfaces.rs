use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Interface {
    pub hwaddr: String,
    pub name: String,

    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub inet: Option<String>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub inet6: Option<String>,
}
