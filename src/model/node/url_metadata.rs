use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UrlMetadata {
    pub filename: String,
    #[serde(rename = "mimetype")]
    pub mime_type: String,
    pub size: u64,
}
