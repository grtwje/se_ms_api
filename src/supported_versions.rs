//! Module for querying the API versions supported by the SolarEdge monitoring server.

use crate::{SendReq, SolaredgeCredentials, MONITORING_API_URL};
use serde::{Deserialize, Serialize};

/// Supported versions request
#[derive(Clone, Debug, PartialEq)]
pub struct SupportedVersionsReq;

/// Supported versions response
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct SupportedVersionsResp {
    /// An array of all the API versions supported by the server
    pub supported: Vec<Release>,
}

/// A release version supported by the server
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Release {
    /// A release number supported by the server in <major.minor.revision> format.
    pub release: String,
}

impl SupportedVersionsReq {
    /// Create a supported versions request message that can be sent to SolarEdge.
    pub fn new() -> Self {
        SupportedVersionsReq {}
    }
}

impl SendReq<SupportedVersionsResp> for SupportedVersionsReq {
    fn build_url(&self, solaredge: &SolaredgeCredentials) -> String {
        format!(
            "{}version/supported?{}",
            *MONITORING_API_URL, solaredge.api_key,
        )
    }
}

impl Default for SupportedVersionsReq {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;

    #[test]
    fn normal_types_unit_test() {
        is_normal::<SupportedVersionsReq>();
        is_normal::<SupportedVersionsResp>();
        is_normal::<Release>();
    }
}
