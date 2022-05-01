//! Module for detailed site energy measurements from meters such as consumption, export (feed-in), import (purchase), etc.

use crate::meter_type::MeterType;
use crate::meter_value::MeterValue;
use crate::time_unit::TimeUnit;
use crate::URL_TIME_FORMAT;
use crate::{Response, SolaredgeCredentials};
use serde::{Deserialize, Serialize};

/// site_energyDetails request
pub struct SiteEnergyDetailedReq {
    start_time: String,
    end_time: String,
    time_unit: String,
    meters: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
/// site_energyDetails response
pub struct SiteEnergyDetailedResp {
    /// Energy details
    pub energyDetails: EnergyDetails,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
/// Energy details
pub struct EnergyDetails {
    /// Granularity of the energy detail values (should match the request)
    pub timeUnit: String,

    /// Measurement unit (e.g. Wh)
    pub unit: String,

    /// For the meter types requested, energy values over the time period
    pub meters: Vec<MeterValue>,
}

impl SiteEnergyDetailedReq {
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
    pub fn new(
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
        time_unit: Option<TimeUnit>,
        meters: Option<Vec<MeterType>>,
    ) -> Self {
        let start_time = format!("startTime={}&", start_time.format(URL_TIME_FORMAT));

        let end_time = format!("endTime={}&", end_time.format(URL_TIME_FORMAT));

        let time_unit = match time_unit {
            Some(t) => format!("timeUnit={}&", t),
            None => "".to_string(),
        };

        let meters = match meters {
            Some(m) => format!(
                "meters={}&",
                m.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            None => "".to_string(),
        };

        SiteEnergyDetailedReq {
            start_time,
            end_time,
            time_unit,
            meters,
        }
    }

    // Send the site_energyDetails request to Solaredge and return the response.
    //
    // # Arguments
    //
    // * `solaredge` - SolarEdge credentials to use for sending
    //
    // # Returns
    // the SolarEdge response or an error string
    pub(crate) fn send(&self, solaredge: &SolaredgeCredentials) -> Result<Response, String> {
        let url = format!(
            "{}site/{}/energyDetails?{}{}{}{}{}",
            solaredge.url_start,
            solaredge.site_id,
            self.meters,
            self.time_unit,
            self.start_time,
            self.end_time,
            solaredge.url_end
        );

        //println!("url: {}\n", url);
        let res = match reqwest::blocking::get(&url) {
            Ok(r) => r,
            Err(e) => return Err(format!("reqwest get error {}", e)),
        };
        //println!("raw response: {:?}", res);

        let parsed = match res.json::<SiteEnergyDetailedResp>() {
            Ok(p) => p,
            Err(e) => return Err(format!("JSON parse error {}", e)),
        };

        Ok(Response::SiteEnergyDetailed(parsed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn site_energy_detailed_req_new_unit_test() {
        let dt = "2022-01-01 00:00:00";
        let ndt = match NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S") {
            Ok(ndt) => ndt,
            Err(_) => panic!("test failed"),
        };
        let req = SiteEnergyDetailedReq::new(ndt, ndt, None, None);
        assert_eq!(req.start_time, format!("startTime={}&", dt));
        assert_eq!(req.end_time, format!("endTime={}&", dt));
        assert_eq!(req.time_unit, "");
        assert_eq!(req.meters, "");
    }
}
