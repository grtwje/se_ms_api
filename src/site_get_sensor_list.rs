//! Module for getting a list of all the sensors in the site, and the device to which they are connected.

use crate::{SendReq, MONITORING_API_URL};
use serde::Deserialize;

/// site_get_sensor_list request
#[derive(Clone, Debug, PartialEq)]
pub struct Req;

/// site_get_sensor_list response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Resp {
    /// The list of sensors installed in the site associated with the gateway they are connected with.
    pub site_sensors: SiteSensors,
}

/// The list of sensors installed in the site associated with the gateway they are connected with.
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SiteSensors {
    /// Number of gateways in the list.
    pub total: u16,

    /// list of gateways
    pub list: Gateways,
}

/// List of gateways
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(transparent)]
pub struct Gateways {
    pub g: Vec<Gateway>,
}

/// Sensor information for a gateway
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Gateway {
    /// Gateway that the sensors are connected to
    pub connected_to: String,

    /// Number of sensors in list
    pub count: u16,

    /// List of sensors
    pub sensors: Sensors,
}

/// List of sensors
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(transparent)]
pub struct Sensors {
    pub s: Vec<Sensor>,
}

/// Information for a single sensor
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Sensor {
    /// Name of the sensor
    pub name: String,

    /// What the sensor measures
    pub measurement: String,

    /// Sensor type
    #[serde(rename = "type")]
    pub sensor_type: String,
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
            "{}equipment/{}/sensors?{}",
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
        is_normal::<SiteSensors>();
        is_normal::<Gateways>();
        is_normal::<Gateway>();
        is_normal::<Sensors>();
        is_normal::<Sensor>();
    }
}
