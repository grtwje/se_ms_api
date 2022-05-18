//! Module for querying the API versions supported by the SolarEdge monitoring server.

use crate::{SendReq, MONITORING_API_URL};
use serde::{Deserialize, Serialize};

/// Supported versions request
#[derive(Clone, Debug, PartialEq)]
pub struct Req;

/// Supported versions response
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Resp {
    /// An array of all the API versions supported by the server
    pub supported: Vec<Release>,
}

/// A release version supported by the server
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Release {
    /// A release number supported by the server in <major.minor.revision> format.
    pub release: String,
}

impl Req {
    /// Create a supported versions request message that can be sent to SolarEdge.
    #[must_use]
    pub fn new() -> Self {
        Req {}
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, _: &str, api_key: &str) -> String {
        format!("{}version/supported?{}", *MONITORING_API_URL, api_key,)
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
        is_normal::<Release>();
    }
}
