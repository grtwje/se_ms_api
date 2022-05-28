//! Module for getting the inventory of SolarEdge equipment in the site,
//! including inverters/SMIs, batteries, meters, gateways and sensors.

use crate::{MeterType, SendReq, MONITORING_API_URL};
use serde::Deserialize;

/// site_inventory request
#[derive(Clone, Debug, PartialEq)]
pub struct Req;

/// site_inventory response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Resp {
    /// List size and list
    pub inventory: Inventory,
}

/// Inventory of site equipment
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    /// List of site meters
    pub meters: Vec<Meter>,

    /// List of site sensors
    pub sensors: Vec<Sensor>,

    /// List of site gateways
    pub gateways: Vec<Gateway>,

    /// List of site batteries
    pub batteries: Vec<Battery>,

    /// List of site inverters
    pub inverters: Vec<Inverter>,
}

/// Meter info
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Meter {
    /// Meter's name
    pub name: String,

    /// Meter's manufacturer
    pub manufacturer: Option<String>,

    /// Meter model
    pub model: Option<String>,

    /// Meter's firmware version
    pub firmware_version: String,

    /// Which SolarEdge device the meter is connected to
    pub connected_to: String,

    /// Which SolarEdge device the meter is connected to
    #[serde(rename = "connectedSolaredgeDeviceSN")]
    pub connected_solaredge_device_sn: String,

    /// Meter type
    #[serde(rename = "type")]
    pub meter_type: MeterType,

    /// Physical for a HW meter or virtual if calculated by arithmetic between other meters
    pub form: String,

    /// Meter serial number
    #[serde(rename = "SN")]
    pub sn: Option<String>,
}

/// Sensor info
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Sensor {
    /// Serial number of connected SolarEdge device
    #[serde(rename = "connectedSolaredgeDeviceSN")]
    pub connected_solaredge_device_sn: String,

    /// Sensor ID
    pub id: String,

    /// Which SolarEdge device the sensor is connected to
    pub connected_to: String,

    /// Sensor category
    pub category: String,

    /// Sensor type
    #[serde(rename = "type")]
    pub sensor_type: String,
}

/// Gateway info
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Gateway {
    /// Gateway name
    pub name: String,

    /// The communication interface used to connect to server. e.g.: Ethernet.
    pub communication_method: String,

    /// Gateway serial number
    #[serde(rename = "SN")]
    pub sn: String,

    /// CPU version
    pub cpu_version: String,
}

/// Battery info
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
    /// Battery's name
    pub name: String,

    /// Battery's manufacturer
    pub manufacturer: String,

    /// Battery model
    pub model: String,

    /// Battery firmware version
    pub firmware_version: String,

    /// Serial number of connected inverter
    pub connected_inverter_sn: String,

    /// The nameplate capacity of the battery as provided by the manufacturer
    pub nameplate_capacity: String,

    /// Battery serial number
    #[serde(rename = "SN")]
    pub sn: String,
}

/// Inverter info
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Inverter {
    /// Inverter's name
    pub name: String,

    /// Inverter's manufacturer
    pub manufacturer: String,

    /// Inverter's model
    pub model: String,

    /// The communication interface used to connect to server. e.g.: Ethernet.
    pub communication_method: String,

    /// CPU version
    pub cpu_version: String,

    /// Inverter serial number
    #[serde(rename = "SN")]
    pub sn: String,

    /// number of optimizers connected to the inverter
    pub connected_optimizers: u32,
}

impl Req {
    /// Create a site environmental benefits request message that can be sent to SolarEdge.
    #[must_use]
    pub fn new() -> Self {
        Req {}
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, site_id: &str, api_key: &str) -> String {
        format!(
            "{}site/{}/inventory?{}",
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
        is_normal::<Inventory>();
        is_normal::<Meter>();
        is_normal::<Sensor>();
        is_normal::<Gateway>();
        is_normal::<Battery>();
        is_normal::<Inverter>();
    }
}
