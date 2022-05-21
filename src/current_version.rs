//! Module for querying the current API version of the SolarEdge monitoring server.

use crate::{SendReq, MONITORING_API_URL};
use serde::Deserialize;

/// Current version request
#[derive(Clone, Debug, PartialEq)]
pub struct Req;

/// Current version response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
pub struct Resp {
    /// The API version running on the server
    pub version: Version,
}

/// The release version of the server
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
pub struct Version {
    /// The release number running on the server in <major.minor.revision> format.
    pub release: String,
}

impl Req {
    /// Create a current version request message that can be sent to SolarEdge.
    #[must_use]
    pub fn new() -> Self {
        Req {}
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, _: &str, api_key: &str) -> String {
        format!("{}version/current?{}", *MONITORING_API_URL, api_key,)
    }
}

impl Default for Req {
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
        is_normal::<Req>();
        is_normal::<Resp>();
        is_normal::<Version>();
    }
}
