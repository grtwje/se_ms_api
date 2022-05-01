//! Module for querying the current API version of the SolarEdge monitoring server.

use crate::{Response, SolaredgeCredentials};
use serde::{Deserialize, Serialize};

/// Current version request
pub struct CurrentVersionReq {}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
/// Current version response
pub struct CurrentVersionResp {
    /// The API version running on the server
    pub version: Version,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
/// The release version of the server
pub struct Version {
    /// The release number running on the server in <major.minor.revision> format.
    pub release: String,
}

impl CurrentVersionReq {
    /// Create a current version request message that can be sent to SolarEdge.
    pub fn new() -> Self {
        CurrentVersionReq {}
    }

    // Send the current version request to Solaredge and return the response.
    //
    // # Arguments
    //
    // * `solaredge` - SolarEdge credentials to use for sending
    //
    // # Returns
    // the SolarEdge response or an error string
    pub(crate) fn send(&self, solaredge: &SolaredgeCredentials) -> Result<Response, String> {
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

        Ok(Response::CurrentVersion(parsed))
    }
}

impl Default for CurrentVersionReq {
    fn default() -> Self {
        Self::new()
    }
}
