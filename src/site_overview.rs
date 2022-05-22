//! Module for site overview requests and responses exchanged with the SolarEdge server monitoring API.

use crate::{SendReq, MONITORING_API_URL};
use serde::Deserialize;

/// site_overview request
#[derive(Clone, Debug, PartialEq)]
pub struct Req;

/// site_overview response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
pub struct Resp {
    /// Overview information about the monitoring site
    pub overview: Overview,
}

/// Overview information for the single site.
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Overview {
    /// Last time the site reported in to SolarEdge.
    pub last_update_time: String,

    /// Total energy produced and revenue of the site
    pub life_time_data: EnergyRevenue,

    /// Energy produced and revenue of the site for the previous year
    pub last_year_data: EnergyRevenue,

    /// Energy produced and revenue of the site for the previous month
    pub last_month_data: EnergyRevenue,

    /// Energy produced and revenue of the site for the previous day
    pub last_day_data: EnergyRevenue,

    /// Power currently being produced by the site
    pub current_power: CurrentPower,

    /// Source of reading
    pub measured_by: String,
}

/// Energy and revenue pair
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnergyRevenue {
    /// Energy value
    pub energy: f32,

    /// Revenue value
    pub revenue: Option<f32>,
}

/// Power currently being produced by the site
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CurrentPower {
    /// Current power reading
    pub power: f32,
}

impl Req {
    /// Create a site details request message that can be sent to SolarEdge.
    #[must_use]
    pub fn new() -> Self {
        Req {}
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, site_id: &str, api_key: &str) -> String {
        format!(
            "{}site/{}/overview?{}",
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
        is_normal::<Overview>();
        is_normal::<EnergyRevenue>();
        is_normal::<CurrentPower>();
    }
}
