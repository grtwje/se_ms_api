//! Module for querying site power flow between all elements of the site including
//! PV array, storage (battery), loads (consumption) and grid.

use crate::{SendReq, MONITORING_API_URL};
use serde::Deserialize;

/// site_power_flow request
#[derive(Clone, Debug, PartialEq)]
pub struct Req;

/// site_power_flow response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    /// Site's current power flow status
    pub site_current_power_flow: SiteCurrentPowerFlow,
}

/// Site's current power flow status
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SiteCurrentPowerFlow {
    /// undocumented
    pub update_refresh_rate: u16,

    /// The measurement units (e.g. Watt)
    pub unit: String,

    /// A table including all the relationships between the elements,
    /// and the power flow directions (producing element and consuming element)
    pub connections: Vec<Connections>,

    /// Electric grid
    #[serde(rename = "GRID")]
    pub grid: Parameters,

    /// Site electricity consumers
    #[serde(rename = "LOAD")]
    pub load: Parameters,

    /// Photovoltaic array
    #[serde(rename = "PV")]
    pub pv: Option<Parameters>,

    /// Electric storage
    #[serde(rename = "STORAGE")]
    pub storage: Option<Parameters>,
}

/// List of producers (from) adn consumers (to) of electricity.
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Connections {
    /// The element providing power
    pub from: String,

    /// The element consuming power
    pub to: String,
}

/// Parameters for each site producer/consumer element.
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    /// The current status of the element (Active / Idle Disabled)
    pub status: String,

    /// The current power of the element. All numbers are positive;
    /// power direction is determined by the “connections” section.
    pub current_power: f32,

    /// (STORAGE only) The accumulated state of energy (% of charge) for all batteries.
    pub charge_level: Option<u16>,

    /// (STORAGE only) If the accumulated storage charge level drops below
    /// a configurable level (currently 10%), this flag is returned.
    pub critical: Option<bool>,

    /// (STORAGE only) In Backup mode (GRID is Disabled), this property
    /// is returned to specify the time left before the storage energy runs out
    /// (estimated according to current load level).
    pub time_left: Option<String>,
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
            "{}site/{}/currentPowerFlow?{}",
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
        is_normal::<SiteCurrentPowerFlow>();
        is_normal::<Connections>();
        is_normal::<Parameters>();
    }
}
