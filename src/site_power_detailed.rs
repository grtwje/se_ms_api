//! Module for detailed site power measurements from meters such as consumption, export (feed-in), import (purchase), etc.

use crate::{MeterType, MeterValue, SendReq, TimeUnit, MONITORING_API_URL, URL_DATE_TIME_FORMAT};
use serde::Deserialize;

/// site_powerDetails request
#[derive(Clone, Debug, PartialEq)]
pub struct Req {
    start_time: String,
    end_time: String,
    meters: String,
}

/// site_powerDetails response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    /// Power details
    pub power_details: PowerDetails,
}

/// Power details
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PowerDetails {
    /// Granularity of the power detail values (should match the request)
    pub time_unit: TimeUnit,

    /// Measurement unit (e.g. Wh)
    pub unit: String,

    /// For the meter types requested, power values over the time period
    pub meters: Vec<MeterValue>,
}

impl Req {
    /// Create an power details request message that can be sent to SolarEdge.
    ///
    /// # Arguments
    ///
    /// * `start_time` - beginning of the time period for the power details
    /// * `end_time`   - end of the time period for the power details
    /// * `meters`     - meter types to collect power details for
    #[must_use]
    pub fn new(
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
        meters: Option<Vec<MeterType>>,
    ) -> Self {
        let start_time = format!("startTime={}&", start_time.format(URL_DATE_TIME_FORMAT));

        let end_time = format!("endTime={}&", end_time.format(URL_DATE_TIME_FORMAT));

        let meters = match meters {
            Some(m) => format!(
                "meters={}&",
                m.iter()
                    .map(MeterType::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            None => "".to_string(),
        };

        Req {
            start_time,
            end_time,
            meters,
        }
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, site_id: &str, api_key: &str) -> String {
        format!(
            "{}site/{}/powerDetails?{}{}{}{}",
            *MONITORING_API_URL, site_id, self.meters, self.start_time, self.end_time, api_key,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;
    use chrono::NaiveDateTime;

    #[test]
    fn site_power_detailed_req_new_unit_test() {
        let dt = "2022-01-01 00:00:00";
        if let Ok(ndt) = NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S") {
            let req = Req::new(ndt, ndt, None);
            assert_eq!(req.start_time, format!("startTime={}&", dt));
            assert_eq!(req.end_time, format!("endTime={}&", dt));
            assert_eq!(req.meters, "");
        } else {
            panic!("test failed");
        }
    }

    #[test]
    fn normal_types_unit_test() {
        is_normal::<Req>();
        is_normal::<Resp>();
        is_normal::<PowerDetails>();
    }
}
