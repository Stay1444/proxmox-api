use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LXCConfiguration {
    pub digest: String,
}
