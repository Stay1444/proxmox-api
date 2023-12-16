use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProxmoxAPIError {
    #[error("Network Error")]
    NetworkError,
    #[error("Error while deserializing response JSON")]
    DeserializationError,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Unknown API error")]
    ApiError(StatusCode),
}

pub type Result<T> = core::result::Result<T, ProxmoxAPIError>;
