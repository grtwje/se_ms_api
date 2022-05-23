//! Module for detailed storage information from batteries: the state of energy, power and lifetime energy.

use crate::{SendReq, MONITORING_API_URL, URL_DATE_TIME_FORMAT};
use serde::Deserialize;

/// site_storage_data request
#[derive(Clone, Debug, PartialEq)]
pub struct Req {
    start_time: String,
    end_time: String,
    serials: String,
}

/// site_storage_data response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    /// Information about the site's storage
    pub storage_data: StorageData,
}

/// Information about the site's storage
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StorageData {
    pub battery_count: u16,
    pub batteries: Batteries,
}

/// Array of batteries
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(transparent)]
pub struct Batteries {
    pub e: Vec<Battery>,
}

/// Data for a single battery
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
    pub nameplate: u32,
}

impl Req {
    /// Create a site storage data request message that can be sent to SolarEdge.
    #[must_use]
    pub fn new(
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
        serials: Option<Vec<String>>,
    ) -> Self {
        let start_time = format!("startTime={}&", start_time.format(URL_DATE_TIME_FORMAT));

        let end_time = format!("endTime={}&", end_time.format(URL_DATE_TIME_FORMAT));

        let serials = match serials {
            Some(s) => format!("serials={}&", s.join(",")),
            None => "".to_string(),
        };

        Req {
            start_time,
            end_time,
            serials,
        }
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, site_id: &str, api_key: &str) -> String {
        format!(
            "{}site/{}/storageData?{}{}{}{}",
            *MONITORING_API_URL, site_id, self.start_time, self.end_time, self.serials, api_key,
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
        is_normal::<StorageData>();
        is_normal::<Batteries>();
        is_normal::<Battery>();
    }
}
