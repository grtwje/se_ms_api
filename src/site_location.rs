//! Module for holding site location data returned in the SolarEdge server monitoring API responses.

use serde::Deserialize;

/// Location of the SolarEdge inverter.
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SiteLocation {
    /// Country of the SolarEdge inverter.
    pub country: String,

    /// State of the SolarEdge inverter.
    pub state: Option<String>, // seems US specific. should this be Option<String>? probably

    /// City of the SolarEdge inverter.
    pub city: String,

    /// Address line 1 of the SolarEdge inverter.
    pub address: String,

    /// Address line 2 of the SolarEdge inverter.
    pub address2: String,

    /// Zip code 1 of the SolarEdge inverter. Used in UK, in EU?
    pub zip: String, // seems US specific. should this be Option<String>?

    /// Time zone of the SolarEdge inverter.
    pub time_zone: String,

    /// Country code (abbreviation) of the SolarEdge inverter.
    pub country_code: String,

    /// State (abbreviation) of the SolarEdge inverter.
    pub state_code: Option<String>, // seems US specific. should this be Option<String>?
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;

    #[test]
    fn normal_types_unit_test() {
        is_normal::<SiteLocation>();
    }
}
