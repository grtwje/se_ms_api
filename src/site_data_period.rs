//! Module for querying the energy production start and end dates of the site.

use crate::{SendReq, MONITORING_API_URL};
use serde::{Deserialize, Serialize};

/// site_data_period request
#[derive(Clone, Debug, PartialEq)]
pub struct Req;

/// site_data_period response
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    /// Period of time site has been producing.
    pub data_period: SiteDataPeriod,
}

/// Period of time site has been producing.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SiteDataPeriod {
    /// Start date of energy production.
    pub start_date: Option<String>,
    /// End date of energy production.
    pub end_date: Option<String>,
}

impl Req {
    /// Create a site_data_period request message that can be sent to SolarEdge.
    #[must_use]
    pub fn new() -> Self {
        Req {}
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, site_id: &str, api_key: &str) -> String {
        format!(
            "{}site/{}/dataPeriod?{}",
            *MONITORING_API_URL, site_id, api_key,
        )
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
        is_normal::<SiteDataPeriod>();
    }
}
