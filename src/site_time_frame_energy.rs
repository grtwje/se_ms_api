//! Module for querying the site total energy produced for a given period.

pub use crate::date_value::DateValue;

use crate::URL_DATE_FORMAT;
use crate::{SendReq, SendReqBulk, MONITORING_API_URL};
use serde::{Deserialize, Serialize};

/// site_time_frame_energy request
#[derive(Clone, Debug, PartialEq)]
pub struct Req {
    start_date: String,
    end_date: String,
}

/// site_time_frame_energy response
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    /// Energy measurements.
    pub time_frame_energy: TimeFrameEnergy,
}

/// Energy measurements.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TimeFrameEnergy {
    /// Energy produced during the time period
    pub energy: f32,

    /// Measurement unit (e.g. Wh)
    pub unit: String,
}

impl Req {
    /// Create a site_time_frame_energy request message that can be sent to SolarEdge.
    #[must_use]
    pub fn new(start_date: chrono::NaiveDate, end_date: chrono::NaiveDate) -> Self {
        let start_date = format!("startDate={}&", start_date.format(URL_DATE_FORMAT));

        let end_date = format!("endDate={}&", end_date.format(URL_DATE_FORMAT));

        Req {
            start_date,
            end_date,
        }
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, site_id: &str, api_key: &str) -> String {
        format!(
            "{}site/{}/timeFrameEnergy?{}{}{}",
            *MONITORING_API_URL, site_id, self.start_date, self.end_date, api_key,
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
        is_normal::<TimeFrameEnergy>();
    }
}
