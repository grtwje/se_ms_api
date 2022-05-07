//! Module for querying the current API version of the SolarEdge monitoring server.

use crate::SolaredgeCredentials;
use serde::{Deserialize, Serialize};

/// Current version request
#[derive(Clone, Debug, PartialEq)]
pub struct CurrentVersionReq;

/// Current version response
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct CurrentVersionResp {
    /// The API version running on the server
    pub version: Version,
}

/// The release version of the server
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct Version {
    /// The release number running on the server in <major.minor.revision> format.
    pub release: String,
}

impl CurrentVersionReq {
    /// Create a current version request message that can be sent to SolarEdge.
    pub fn new() -> Self {
        CurrentVersionReq {}
    }

    /// Send the current version request to Solaredge and return the response.
    ///
    /// # Arguments
    ///
    /// * `solaredge` - SolarEdge credentials to use for sending
    ///
    /// # Returns
    /// the SolarEdge response or an error string
    pub fn send(&self, solaredge: &SolaredgeCredentials) -> Result<CurrentVersionResp, String> {
        let url = format!(
            "{}version/current?{}",
            solaredge.url_start, solaredge.url_end
        );

        let res = match reqwest::blocking::get(&url) {
            Ok(r) => r,
            Err(e) => return Err(format!("reqwest get error {}", e)),
        };

        let parsed = match res.json::<CurrentVersionResp>() {
            Ok(p) => p,
            Err(e) => return Err(format!("JSON parse error {}", e)),
        };

        Ok(parsed)
    }
}

impl Default for CurrentVersionReq {
    fn default() -> Self {
        Self::new()
    }
}
