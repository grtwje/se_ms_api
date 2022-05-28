//! Module for getting a list of equipment component replacements ordered by date.
//! This method is applicable to inverters, optimizers, batteries and gateways

use crate::{SendReq, MONITORING_API_URL};
use serde::Deserialize;

/// site_equipment_change_log request
#[derive(Clone, Debug, PartialEq)]
pub struct Req {
    serial_number: String,
}

/// site_equipment_change_log response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Resp {
    /// Equipment change history
    pub change_log: ChangeLog,
}

/// Equipment change history
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLog {
    /// Number of entries in the change list
    pub count: u32,

    /// List of changes for the equipment
    pub list: Vec<ChangeEntry>,
}

/// Equipment change record
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangeEntry {
    /// Equipment short serial number
    pub serial_number: String,

    /// Inverter/battery/optimizer/gateway model
    pub part_number: String,

    /// Date of replacement of that equipment component
    pub date: String,
}

impl Req {
    /// Create an power details request message that can be sent to SolarEdge.
    ///
    /// # Arguments
    ///
    /// * `serial_number` - Inverter, battery, optimizer or gateway short serial number
    #[must_use]
    pub fn new(serial_number: &str) -> Self {
        Req {
            serial_number: serial_number.to_string(),
        }
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, site_id: &str, api_key: &str) -> String {
        format!(
            "{}equipment/{}/{}/changeLog?{}",
            *MONITORING_API_URL, site_id, self.serial_number, api_key,
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
        is_normal::<ChangeLog>();
        is_normal::<ChangeEntry>();
    }
}
