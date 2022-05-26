//! Library to retrieve data from the SolarEdge Monitoring Server.
//!
//! Based on the API define here:
//! <https://www.solaredge.com/sites/default/files/se_monitoring_api.pdf>,
//! released January 2022.
//!
//! The basic use case is:
//!  1) Create a SolarEdge struct that contains the site id and api key
//!     that will be used for the requests. (The se_monitoring_api.pdf linked
//!     above has instructions for getting your site id and api key.
//!  2) Create a request for the information that you want.
//!  3) Send the request using the SolarEdge struct.
//!  4) Read the response to get the information.
//!
//! ```no_run
//! extern crate se_ms_api;
//! use se_ms_api::{SendReq, SiteDetailsReq, SolaredgeCredentials};
//!
//! let site_id = "my_site_id";
//! let api_key = "my_api_key";
//!
//! let cred = SolaredgeCredentials::new(&site_id, &api_key); // (1)
//! let req = SiteDetailsReq::new();                          // (2)
//! let resp = req.send(&cred);                               // (3)
//!
//! match resp {                                              // (4)
//!    Ok(r) => {
//!        println!("My site's status is {}.", r.details.status);
//!    }
//!    Err(e) => {
//!        panic!("Unexpected SiteDetails response: {:?}", e);
//!    }
//!}
//! ```
//! Due to the restrictions that SolarEdge imposes on this API, this library
//! does not try to be performant. For example, it makes blocking HTTP requests.
//!
//! Supported API requests/responses include:
//! * [CurrentVersionReq] / [CurrentVersionResp]
//! * [SiteDataPeriodReq] / [SiteDataPeriodResp]
//! * [SiteDetailsReq] / [SiteDetailsResp]
//! * [SiteEnergyReq] / [SiteEnergyResp]
//! * [SiteEnergyDetailedReq] / [SiteEnergyDetailedResp]
//! * [SiteEnvironmentalBenefitsReq] / [SiteEnvironmentalBenefitsResp]
//! * [SiteEquipmentListReq] / [SiteEquipmentListResp]
//! * [SiteGetMetersDataReq] / [SiteGetMetersDataResp]
//! * [SiteGetSensorListReq] / [SiteGetSensorListResp]
//! * [SiteListReq] / [SiteListResp]
//! * [SiteOverviewReq] / [SiteOverviewResp]
//! * [SitePowerReq] / [SitePowerResp]
//! * [SitePowerDetailedReq] / [SitePowerDetailedResp]
//! * [SitePowerFlowReq] / [SitePowerFlowResp]
//! * [SiteStorageDataReq] / [SiteStorageDataResp]
//! * [SiteTimeFrameEnergyReq] / [SiteTimeFrameEnergyResp]
//! * [SupportedVersionsReq] / [SupportedVersionsResp]
//!
//! TODO:
//! * SiteInventory,
//! * SiteInverterTechnicalData,
//! * SiteEquipmentChangeLog,
//! * AccountsList
//!
//! Unsupported API requests/responses include:
//! * SiteImage,
//! * SiteInstallerImage,
//! * SiteSensorData

#![warn(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::doc_markdown)]

pub use current_version::{Req as CurrentVersionReq, Resp as CurrentVersionResp};
pub use site_data_period::{Req as SiteDataPeriodReq, Resp as SiteDataPeriodResp};
pub use site_details::{Req as SiteDetailsReq, Resp as SiteDetailsResp};
pub use site_energy::{Req as SiteEnergyReq, Resp as SiteEnergyResp};
pub use site_energy_detailed::{Req as SiteEnergyDetailedReq, Resp as SiteEnergyDetailedResp};
pub use site_environmental_benefits::{
    Req as SiteEnvironmentalBenefitsReq, Resp as SiteEnvironmentalBenefitsResp,
};
pub use site_equipment_list::{Req as SiteEquipmentListReq, Resp as SiteEquipmentListResp};
pub use site_get_meters_data::{Req as SiteGetMetersDataReq, Resp as SiteGetMetersDataResp};
pub use site_get_sensor_list::{Req as SiteGetSensorListReq, Resp as SiteGetSensorListResp};
pub use site_list::{Req as SiteListReq, Resp as SiteListResp};
pub use site_overview::{Req as SiteOverviewReq, Resp as SiteOverviewResp};
pub use site_power::{Req as SitePowerReq, Resp as SitePowerResp};
pub use site_power_detailed::{Req as SitePowerDetailedReq, Resp as SitePowerDetailedResp};
pub use site_power_flow::{Req as SitePowerFlowReq, Resp as SitePowerFlowResp};
pub use site_storage_data::{Req as SiteStorageDataReq, Resp as SiteStorageDataResp};
pub use site_time_frame_energy::{Req as SiteTimeFrameEnergyReq, Resp as SiteTimeFrameEnergyResp};
pub use supported_versions::{Req as SupportedVersionsReq, Resp as SupportedVersionsResp};

pub use date_value::DateValue;
pub use error::{Error, Kind};
pub use meter_type::MeterType;
pub use meter_value::MeterValue;
use serde::Deserialize;
pub use site_details::SiteDetails;
pub use site_location::SiteLocation;
pub use site_module::SiteModule;
pub use site_public_settings::SitePublicSettings;
pub use system_units::SystemUnits;
pub use time_unit::TimeUnit;

mod current_version;
mod date_value;
mod error;
mod meter_type;
mod meter_value;
mod site_data_period;
mod site_details;
mod site_energy;
mod site_energy_detailed;
mod site_environmental_benefits;
mod site_equipment_list;
mod site_get_meters_data;
mod site_get_sensor_list;
mod site_list;
mod site_location;
mod site_module;
mod site_overview;
mod site_power;
mod site_power_detailed;
mod site_power_flow;
mod site_public_settings;
mod site_storage_data;
mod site_time_frame_energy;
mod supported_versions;
mod system_units;
mod time_unit;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref REQWEST_CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
    static ref MONITORING_API_URL: String = "https://monitoringapi.solaredge.com/".to_string();
}

const URL_DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const URL_DATE_FORMAT: &str = "%Y-%m-%d";

/// Struct for accessing SolarEdge's monitoring server for a given site and api key.
///
/// Used as the parameter for the send() function of all of the possible requests.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SolaredgeCredentials {
    site_id: String,
    api_key: String,
}

impl SolaredgeCredentials {
    /// Create a Solaredge destination for the requests from the given site id and api_key.
    ///
    /// # Arguments
    ///
    /// * `site_id` - ID used by SolarEdge to identify your site.
    /// * `api_key` - API token used SolarEdge to authenticate user is allowed to access site data.
    ///
    /// # Returns
    /// A credentials struct to be used in subsequent request sends.
    #[must_use]
    pub fn new(site_id: &str, api_key: &str) -> Self {
        let site_id = site_id.to_string();
        let api_key = format!("api_key={}", api_key);

        SolaredgeCredentials { site_id, api_key }
    }

    /// See the site ID being used in the credentials.
    /// Used by the integration test framework.
    #[must_use]
    pub fn site_id(&self) -> &str {
        &self.site_id
    }
}

/// All Solaredge requests implement this trait since sending the request
/// and getting the response is the same for all requests.
pub trait SendReq<Resp> {
    #[doc(hidden)]
    fn build_url(&self, site_id: &str, api_key: &str) -> String;

    /// Send the request to Solaredge and return the response.
    ///
    /// # Arguments
    ///
    /// * `solaredge` - SolarEdge credentials to use for sending
    ///
    /// # Returns
    /// The SolarEdge response or an error string.
    ///
    /// # Errors
    /// Errors can occur on the request send or when parsing the response.
    fn send(&self, solaredge: &SolaredgeCredentials) -> Result<Resp, Error>
    where
        for<'de> Resp: Deserialize<'de>,
    {
        let url = self.build_url(&solaredge.site_id, &solaredge.api_key);

        let res = REQWEST_CLIENT.get(url).send()?;

        if res.status().is_success() {
            let parsed = res.json::<Resp>()?;

            Ok(parsed)
        } else {
            let reason = match res.status().canonical_reason() {
                Some(r) => r.to_string(),
                None => res.status().as_str().to_string(),
            };

            let text = match res.text() {
                Ok(t) => t,
                Err(_) => "".to_string(),
            };

            Err(Error::new(Kind::HttpErrorStatus(reason, text)))
        }
    }
}

#[cfg(test)]
pub(crate) fn is_normal<T: Sized + Send + Sync + Unpin>() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solaredge_credentials_unit_test() {
        let se = SolaredgeCredentials::new("id", "key");
        assert_eq!(se.site_id, "id");
        assert_eq!(se.site_id(), "id");
        assert_eq!(se.api_key, "api_key=key");
    }

    #[test]
    fn normal_types_unit_test() {
        is_normal::<SolaredgeCredentials>();
    }
}
