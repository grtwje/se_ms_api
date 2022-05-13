//! Module for querying the current API version of the SolarEdge monitoring server.

use crate::{SendReq, SolaredgeCredentials, MONITORING_API_URL};
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
}

impl SendReq<CurrentVersionResp> for CurrentVersionReq {
    fn build_url(&self, solaredge: &SolaredgeCredentials) -> String {
        format!(
            "{}version/current?{}",
            *MONITORING_API_URL, solaredge.api_key,
        )
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
