//! Module for reporting solar panel module information from the SolarEdge server monitoring API.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
/// Solar panel module information
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
