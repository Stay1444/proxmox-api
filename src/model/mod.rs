use reqwest::{Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::error;

use crate::error::ProxmoxAPIError;

pub mod cluster;
pub mod node;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PveResponse<T> {
    pub data: T,
}

impl<T> PveResponse<T>
where
    T: DeserializeOwned,
{
    pub(crate) async fn from_response(
        response: Response,
    ) -> Result<PveResponse<T>, ProxmoxAPIError> {
        if !response.status().is_success() {
            match response.status() {
                StatusCode::UNAUTHORIZED => return Err(ProxmoxAPIError::Unauthorized),
                status => return Err(ProxmoxAPIError::ApiError(status)),
            }
        }

        let body = response
            .text()
            .await
            .map_err(|_| ProxmoxAPIError::NetworkError)?;

        dbg!(&body);

        serde_json::from_str(&body).map_err(|err| {
            error!("{err}");
            ProxmoxAPIError::DeserializationError
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PveVersion {
    pub release: String, // The current Proxmox VE point release in `x.y` format.
    pub repoid: String,  // The short git revision from which this version was build.
    pub version: String, // The full pve-manager package version of this node.
    #[serde(default)]
    pub console: Option<PveConsoleViewer>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PveConsoleViewer {
    Applet,
    VV,
    Html5,
    XTermJS,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    Amd64,
    I386,
    Arm64,
    ArmHF,
    RiscV32,
    RiscV64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum OSType {
    Debian,
    Devuan,
    Ubuntu,
    CentOS,
    Fedora,
    OpenSUSE,
    ArchLinux,
    Alpine,
    Gentoo,
    NixOS,
    Unmanaged,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Size {
    TB(f64),
    GB(f64),
    MB(f64),
    KB(f64),
    B(u64),
}

impl ToString for Size {
    fn to_string(&self) -> String {
        match self {
            Size::TB(q) => format!("{q}T"),
            Size::GB(q) => format!("{q}G"),
            Size::MB(q) => format!("{q}M"),
            Size::KB(q) => format!("{q}K"),
            Size::B(q) => format!("{q}"),
        }
    }
}

impl Size {
    pub fn to_tb(&self) -> f64 {
        match self {
            Size::TB(q) => *q,
            Size::GB(q) => *q / 1024.,
            Size::MB(q) => *q / 1024. / 1024.,
            Size::KB(q) => *q / 1024. / 1024. / 1024.,
            Size::B(q) => *q as f64 / 1024. / 1024. / 1024. / 1024.,
        }
    }

    pub fn to_gb(&self) -> f64 {
        match self {
            Size::TB(q) => *q * 1024.,
            Size::GB(q) => *q,
            Size::MB(q) => *q / 1024.,
            Size::KB(q) => *q / 1024. / 1024.,
            Size::B(q) => *q as f64 / 1024.0 / 1024.0 / 1024.0,
        }
    }

    pub fn to_mb(&self) -> f64 {
        match self {
            Size::TB(q) => *q * 1024. * 1024.,
            Size::GB(q) => *q * 1024.,
            Size::MB(q) => *q,
            Size::KB(q) => *q / 1024.,
            Size::B(q) => *q as f64 / 1024. / 1024.,
        }
    }

    pub fn to_kb(&self) -> f64 {
        match self {
            Size::TB(q) => *q * 1024. * 1024. * 1024.,
            Size::GB(q) => *q * 1024. * 1024.,
            Size::MB(q) => *q * 1024.,
            Size::KB(q) => *q,
            Size::B(q) => *q as f64 / 1024.,
        }
    }

    pub fn to_bytes(&self) -> u64 {
        match self {
            Size::TB(q) => (*q * 1024. * 1024. * 1024. * 1024.) as u64,
            Size::GB(q) => (*q * 1024. * 1024. * 1024.) as u64,
            Size::MB(q) => (*q * 1024. * 1024.) as u64,
            Size::KB(q) => (*q * 1024.) as u64,
            Size::B(q) => *q,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Size;

    #[test]
    pub fn test_size_tb() {
        let size = Size::TB(1.0);

        assert_eq!(size.to_tb(), 1.0);
        assert_eq!(size.to_gb(), 1024.0);
        assert_eq!(size.to_mb(), 1048576.0);
        assert_eq!(size.to_kb(), 1073741824.0);
        assert_eq!(size.to_bytes(), 1099511627776);
    }

    #[test]
    pub fn test_size_gb() {
        let size = Size::GB(1.0);

        assert_eq!(size.to_tb(), 0.0009765625);
        assert_eq!(size.to_gb(), 1.0);
        assert_eq!(size.to_mb(), 1024.0);
        assert_eq!(size.to_kb(), 1048576.0);
        assert_eq!(size.to_bytes(), 1073741824);
    }

    #[test]
    pub fn test_size_mb() {
        let size = Size::MB(1.0);

        assert_eq!(size.to_tb(), 0.00000095367431640625);
        assert_eq!(size.to_gb(), 0.0009765625);
        assert_eq!(size.to_mb(), 1.0);
        assert_eq!(size.to_kb(), 1024.0);
        assert_eq!(size.to_bytes(), 1048576);
    }

    #[test]
    pub fn test_size_kb() {
        let size = Size::KB(1.0);

        assert_eq!(size.to_tb(), 0.0000000009313225746154785);
        assert_eq!(size.to_gb(), 0.00000095367431640625);
        assert_eq!(size.to_mb(), 0.0009765625);
        assert_eq!(size.to_kb(), 1.0);
        assert_eq!(size.to_bytes(), 1024);
    }

    #[test]
    pub fn test_size_bytes() {
        let size = Size::B(1);

        assert_eq!(size.to_tb(), 0.0000000000009094947017729282);
        assert_eq!(size.to_gb(), 0.0000000009313225746154785);
        assert_eq!(size.to_mb(), 0.00000095367431640625);
        assert_eq!(size.to_kb(), 0.0009765625);
        assert_eq!(size.to_bytes(), 1);
    }
}
