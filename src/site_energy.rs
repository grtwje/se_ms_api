//! Module for querying the site energy measurements.

use crate::{DateValue, SendReq, TimeUnit, MONITORING_API_URL, URL_DATE_FORMAT};
use serde::{Deserialize, Serialize};

/// site_energy request
#[derive(Clone, Debug, PartialEq)]
pub struct Req {
    start_date: String,
    end_date: String,
    time_unit: String,
}

/// site_energy response
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    /// Energy measurements.
    pub energy: Energy,
}

/// Energy measurements.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Energy {
    /// Granularity of the energy measurements (should match the request)
    pub time_unit: TimeUnit,

    /// Measurement unit (e.g. Wh)
    pub unit: String,

    /// For the dates requested, measurements over the time period
    pub values: Vec<DateValue>,
}

impl Req {
    /// Create a site_energy request message that can be sent to SolarEdge.
    ///
    /// # Arguments
    ///
    /// * `start_date` - beginning date for the energy details
    /// * `end_date`   - end date for the energy details
    /// * `time_unit`  - size of time unit to collect over the date period
    #[must_use]
    pub fn new(
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
        time_unit: Option<TimeUnit>,
    ) -> Self {
        let start_date = format!("startDate={}&", start_date.format(URL_DATE_FORMAT));

        let end_date = format!("endDate={}&", end_date.format(URL_DATE_FORMAT));

        let time_unit = match time_unit {
            Some(t) => format!("timeUnit={}&", t),
            None => "".to_string(),
        };

        Req {
            start_date,
            end_date,
            time_unit,
        }
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, site_id: &str, api_key: &str) -> String {
        format!(
            "{}site/{}/energy?{}{}{}{}",
            *MONITORING_API_URL, site_id, self.time_unit, self.start_date, self.end_date, api_key,
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
        is_normal::<Energy>();
    }
}
