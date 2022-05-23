//! Module for getting all environmental benefits based on site energy production:
//! CO2 emissions saved, equivalent trees planted, and light bulbs powered for a day.

use crate::{SendReq, SystemUnits, MONITORING_API_URL};
use serde::Deserialize;

/// site_environmental_benefits request
#[derive(Clone, Debug, PartialEq)]
pub struct Req {
    /// The measurement system to use in the response.
    system_units: String,
}

/// site_environmental_benefits response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    /// Environmental benefits of the monitoring site
    pub env_benefits: EnvBenefits,
}

/// Environmental benefits of the monitoring site
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnvBenefits {
    /// Quantity of CO2 emissions that would have been generated by an equivalent fossil fuel system.
    pub gas_emission_saved: GasEmissionSaved,

    /// Equivalent planting of new trees for reducing CO2 levels.
    pub trees_planted: f32,

    /// Number of light bulbs that could have been powered by the site for a day.
    pub light_bulbs: f32,
}

/// Environmental benefits of the monitoring site
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GasEmissionSaved {
    /// Measurement unit of following gases.
    pub units: String,

    /// Carbon Dioxide
    pub co2: f32,

    /// Sulphur Dioxide
    pub so2: f32,

    /// Nitrous Oxide
    pub nox: f32,
}

impl Req {
    /// Create a site environmental benefits request message that can be sent to SolarEdge.
    #[must_use]
    pub fn new(system_units: Option<SystemUnits>) -> Self {
        let system_units = match system_units {
            Some(su) => format!("systemUnits={}&", su),
            None => "".to_string(),
        };

        Req { system_units }
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, site_id: &str, api_key: &str) -> String {
        format!(
            "{}site/{}/envBenefits?{}{}",
            *MONITORING_API_URL, site_id, self.system_units, api_key,
        )
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
        is_normal::<EnvBenefits>();
        is_normal::<GasEmissionSaved>();
    }
}