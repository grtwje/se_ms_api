//! Module for querying the API versions supported by the SolarEdge monitoring server.

use crate::{Response, SolaredgeCredentials};
use serde::{Deserialize, Serialize};

/// Supported versions request
pub struct SupportedVersionsReq {}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
/// Supported versions response
pub struct SupportedVersionsResp {
    /// An array of all the API versions supported by the server
    pub supported: Vec<Release>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
/// A release version supported by the server
pub struct Release {
    /// A release number supported by the server in <major.minor.revision> format.
    pub release: String,
}

impl SupportedVersionsReq {
    /// Create a supported versions request message that can be sent to SolarEdge.
    pub fn new() -> Self {
        SupportedVersionsReq {}
    }

    // Send the supported versions request to Solaredge and return the response.
    //
    // # Arguments
    //
    // * `solaredge` - SolarEdge credentials to use for sending
    //
    // # Returns
    // the SolarEdge response or an error string
    pub(crate) fn send(&self, solaredge: &SolaredgeCredentials) -> Result<Response, String> {
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

        Ok(Response::SupportedVersions(parsed))
    }
}

impl Default for SupportedVersionsReq {
    fn default() -> Self {
        Self::new()
    }
}
