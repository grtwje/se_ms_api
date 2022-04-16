//! Module for holding site location data returned in the SolarEdge server monitoring API responses.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
/// Location of the SolarEdge inverter.
pub struct SiteLocation {
    /// Country of the SolarEdge inverter.
    pub country: String,

    /// State of the SolarEdge inverter.
    pub state: String, // seems US specific. should this be Option<String>?

    /// City of the SolarEdge inverter.
    pub city: String,

    /// Address line 1 of the SolarEdge inverter.
    pub address: String,

    /// Address line 2 of the SolarEdge inverter.
    pub address2: String,

    /// Zip code 1 of the SolarEdge inverter.
    pub zip: String, // seems US specific. should this be Option<String>?

    /// Time zone of the SolarEdge inverter.
    pub timeZone: String,

    /// Country code (abbreviation) of the SolarEdge inverter.
    pub countryCode: String,

    /// State (abbreviation) of the SolarEdge inverter.
    pub stateCode: String, // seems US specific. should this be Option<String>?
}
