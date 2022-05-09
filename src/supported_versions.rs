//! Module for querying the API versions supported by the SolarEdge monitoring server.

use crate::error::Error;
use crate::{SolaredgeCredentials, REQWEST_CLIENT};
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

    /// Send the supported versions request to Solaredge and return the response.
    ///
    /// # Arguments
    ///
    /// * `solaredge` - SolarEdge credentials to use for sending
    ///
    /// # Returns
    /// The SolarEdge response or an error string.
    /// Errors can occur on the request send or when parsing the response.
    pub fn send(&self, solaredge: &SolaredgeCredentials) -> Result<SupportedVersionsResp, Error> {
        let url = format!(
            "{}version/supported?{}",
            solaredge.url_start, solaredge.url_end
        );

        let res = REQWEST_CLIENT.get(&url).send()?;

        let parsed = res.json::<SupportedVersionsResp>()?;

        Ok(parsed)
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
