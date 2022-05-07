//! Module for reporting solar panel module information from the SolarEdge server monitoring API.

use serde::{Deserialize, Serialize};

/// Solar panel module information
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct SiteModule {
    /// solar panel manufacturer
    pub manufacturerName: String,

    /// solar panel model name/number
    pub modelName: String,

    /// solar panel max output power
    pub maximumPower: f32,

    /// solar panel temperature coefficient
    pub temperatureCoef: f32,
}
