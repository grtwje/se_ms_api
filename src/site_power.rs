//! Module for querying the site power measurements in 15 minute resolution.

use crate::{DateValue, SendReq, SendReqBulk, TimeUnit, MONITORING_API_URL, URL_DATE_TIME_FORMAT};
use serde::{Deserialize, Serialize};

/// site_power request
#[derive(Clone, Debug, PartialEq)]
pub struct Req {
    start_time: String,
    end_time: String,
}

/// site_power response
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    /// Power measurements.
    pub power: Power,
}

/// Power measurements.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Power {
    /// Time unit of the Power measurements
    pub time_unit: TimeUnit,

    /// Measurement unit (e.g. W)
    pub unit: String,

    /// For the dates requested, measurements over the time period
    pub values: Vec<DateValue>,
}

impl Req {
    /// Create a site_power request message that can be sent to SolarEdge.
    ///     
    /// # Arguments
    ///
    /// * `start_time` - beginning of the time period for the energy details
    /// * `end_time`   - end of the time period for the energy details
    #[must_use]
    pub fn new(start_time: chrono::NaiveDateTime, end_time: chrono::NaiveDateTime) -> Self {
        let start_time = format!("startTime={}&", start_time.format(URL_DATE_TIME_FORMAT));

        let end_time = format!("endTime={}&", end_time.format(URL_DATE_TIME_FORMAT));

        Req {
            start_time,
            end_time,
        }
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, site_id: &str, api_key: &str) -> String {
        format!(
            "{}site/{}/power?{}{}{}",
            *MONITORING_API_URL, site_id, self.start_time, self.end_time, api_key,
        )
    }
}

impl SendReqBulk<Resp> for Req {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;

    #[test]
    fn normal_types_unit_test() {
        is_normal::<Req>();
        is_normal::<Resp>();
        is_normal::<Power>();
    }
}
