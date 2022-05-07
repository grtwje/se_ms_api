//! Module for querying the current API version of the SolarEdge monitoring server.

use crate::SolaredgeCredentials;
use serde::{Deserialize, Serialize};

/// Current version request
#[derive(Clone, Debug, PartialEq)]
pub struct CurrentVersionReq;

/// Current version response
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct CurrentVersionResp {
    /// The API version running on the server
    pub version: Version,
}

/// The release version of the server
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
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
    /// The SolarEdge response or an error string.
    /// Errors can occur on the request send or when parsing the response.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;

    #[test]
    fn normal_types_unit_test() {
        is_normal::<CurrentVersionReq>();
        is_normal::<CurrentVersionResp>();
        is_normal::<Version>();
    }
}
