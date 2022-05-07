//! Module for querying the API versions supported by the SolarEdge monitoring server.

use crate::SolaredgeCredentials;
use serde::{Deserialize, Serialize};

/// Supported versions request
#[derive(Clone, Debug, PartialEq)]
pub struct SupportedVersionsReq;

/// Supported versions response
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct SupportedVersionsResp {
    /// An array of all the API versions supported by the server
    pub supported: Vec<Release>,
}

/// A release version supported by the server
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct Release {
    /// A release number supported by the server in <major.minor.revision> format.
    pub release: String,
}

impl SupportedVersionsReq {
    /// Create a supported versions request message that can be sent to SolarEdge.
    pub fn new() -> Self {
        SupportedVersionsReq {}
    }

    /// Send the supported versions request to Solaredge and return the response.
    ///
    /// # Arguments
    ///
    /// * `solaredge` - SolarEdge credentials to use for sending
    ///
    /// # Returns
    /// the SolarEdge response or an error string
    pub fn send(&self, solaredge: &SolaredgeCredentials) -> Result<SupportedVersionsResp, String> {
        let url = format!(
            "{}version/supported?{}",
            solaredge.url_start, solaredge.url_end
        );

        let res = match reqwest::blocking::get(&url) {
            Ok(r) => r,
            Err(e) => return Err(format!("reqwest get error {}", e)),
        };

        let parsed = match res.json::<SupportedVersionsResp>() {
            Ok(p) => p,
            Err(e) => return Err(format!("JSON parse error {}", e)),
        };

        Ok(parsed)
    }
}

impl Default for SupportedVersionsReq {
    fn default() -> Self {
        Self::new()
    }
}
