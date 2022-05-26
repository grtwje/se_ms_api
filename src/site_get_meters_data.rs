//! Module for each meter on site its lifetime energy reading, metadata and the device to which it’s connected to.

use crate::{DateValue, MeterType, SendReq, TimeUnit, MONITORING_API_URL, URL_DATE_TIME_FORMAT};
use serde::Deserialize;

/// site_get_meters_data request
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Req {
    start_time: String,
    end_time: String,
    time_unit: String,
    meters: String,
}

/// site_get_meters_data response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    /// Meter energy details
    pub meter_energy_details: MeterEnergyDetails,
}

/// Meter energy details
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MeterEnergyDetails {
    /// Granularity of the energy detail values (should match the request)
    pub time_unit: TimeUnit,

    /// Measurement unit (e.g. Wh)
    pub unit: String,

    /// For the meter types requested, meter info and energy values over the time period
    pub meters: Vec<Meter>,
}

/// Meter details
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Meter {
    /// Serial number of the meter
    pub meter_serial_number: String,

    /// Inverter to which the meter is connected to
    #[serde(rename = "connectedSolaredgeDeviceSN")]
    pub connected_solaredge_device_sn: String,

    /// Meter model
    pub model: String,

    /// Production, Consumption, FeedIn or Purchased
    pub meter_type: MeterType,

    /// energy values over the time period
    pub values: Vec<DateValue>,
}

impl Req {
    /// Create an energy details request message that can be sent to SolarEdge.
    ///
    /// # Arguments
    ///
    /// * `start_time` - beginning of the time period for the energy details
    /// * `end_time`   - end of the time period for the energy details
    /// * `time_unit`  - aggregation granularity
    ///                  For the time period requested, energy detail values will be
    ///                  chunked into units of this size.
    /// * `meters`     - meter types to collect energy details for
    #[must_use]
    pub fn new(
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
        time_unit: Option<TimeUnit>,
        meters: Option<Vec<MeterType>>,
    ) -> Self {
        let start_time = format!("startTime={}&", start_time.format(URL_DATE_TIME_FORMAT));

        let end_time = format!("endTime={}&", end_time.format(URL_DATE_TIME_FORMAT));

        let time_unit = match time_unit {
            Some(t) => format!("timeUnit={}&", t),
            None => "".to_string(),
        };

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
            time_unit,
            meters,
        }
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, site_id: &str, api_key: &str) -> String {
        format!(
            "{}site/{}/meters?{}{}{}{}{}",
            *MONITORING_API_URL,
            site_id,
            self.meters,
            self.time_unit,
            self.start_time,
            self.end_time,
            api_key,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;
    use chrono::NaiveDateTime;

    #[test]
    fn site_get_meters_data_req_new_unit_test() {
        let dt = "2022-01-01 00:00:00";
        if let Ok(ndt) = NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S") {
            let req = Req::new(ndt, ndt, None, None);
            assert_eq!(req.start_time, format!("startTime={}&", dt));
            assert_eq!(req.end_time, format!("endTime={}&", dt));
            assert_eq!(req.time_unit, "");
            assert_eq!(req.meters, "");
        } else {
            panic!("test failed");
        }
    }

    #[test]
    fn normal_types_unit_test() {
        is_normal::<Req>();
        is_normal::<Resp>();
        is_normal::<MeterEnergyDetails>();
        is_normal::<Meter>();
    }
}
