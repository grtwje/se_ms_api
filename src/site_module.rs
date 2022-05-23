//! Module for reporting solar panel module information from the SolarEdge server monitoring API.

use serde::Deserialize;

/// Solar panel module information
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SiteModule {
    /// solar panel manufacturer
    pub manufacturer_name: String,

    /// solar panel model name/number
    pub model_name: String,

    /// solar panel max output power
    pub maximum_power: f32,

    /// solar panel temperature coefficient
    pub temperature_coef: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;

    #[test]
    fn normal_types_unit_test() {
        is_normal::<SiteModule>();
    }
}
