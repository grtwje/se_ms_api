//! Module for getting a list of inverters/SMIs in the specific site.

use crate::{SendReq, MONITORING_API_URL};
use serde::Deserialize;

/// site_equipment_list request
#[derive(Clone, Debug, PartialEq)]
pub struct Req;

/// site_equipment_list response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    /// List size and list
    pub reporters: Reporters,
}

/// List size and list
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Reporters {
    /// Number of entries in the equipment list
    pub count: u16,

    /// List of equipment
    pub list: EquipmentList,
}

/// List of equipment
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(transparent)]
pub struct EquipmentList {
    pub eq: Vec<Equipment>,
}

/// Details on a single piece of equipment
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Equipment {
    /// Equipment's name
    pub name: String,

    /// Equipment's manufacturer
    pub manufacturer: String,

    /// Equipment model
    pub model: String,

    /// Equipment's serial number
    pub serial_number: String,

    #[serde(rename = "kWpDC")]
    pub kw_pdc: Option<String>,
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
            "{}equipment/{}/list?{}",
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
        is_normal::<Reporters>();
        is_normal::<EquipmentList>();
        is_normal::<Equipment>();
    }
}
