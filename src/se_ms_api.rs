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
//! * [CurrentVersionReq]/[CurrentVersionResp]
//! * [SiteDetailsReq] / [SiteDetailsResp]
//! * [SiteEnergyDetailedReq] / [SiteEnergyDetailedResp]
//! * [SupportedVersionsReq]/[SupportedVersionsResp]
//!
//! TODO:
//! SitesList,
//! SiteDataPeriod start/end dates,
//! SiteDataPeriod bulk,
//! SiteEnergy,
//! SiteEnergy bulk,
//! SiteTimeFrameEnergy,
//! SiteTimeFrameEnergy bulk,
//! SitePower,
//! SitePower bulk,
//! SiteOverview,
//! SiteOverview bulk,
//! SitePowerDetailed,
//! SitePowerFlow,
//! SiteStorageInformation,
//! SiteImage,
//! SiteEnvironmentalBenefits,
//! SiteInstallerImage,
//! SiteEquipmentList,
//! SiteInventory,
//! SiteInverterTechnicalData,
//! SiteEquipmentChangeLog,
//! AccountsList,
//! SiteMetersData,
//! SiteSensorList,
//! SiteSensorData

#![warn(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

pub use current_version::{CurrentVersionReq, CurrentVersionResp};
pub use error::{Error, ErrorKind};
pub use meter_type::MeterType;
use serde::Deserialize;
pub use site_details::{SiteDetailsReq, SiteDetailsResp};
pub use site_energy_detailed::{SiteEnergyDetailedReq, SiteEnergyDetailedResp};
pub use supported_versions::{SupportedVersionsReq, SupportedVersionsResp};

mod current_version;
mod date_value;
mod error;
mod meter_type;
mod meter_value;
mod site_details;
mod site_energy_detailed;
mod site_location;
mod site_module;
mod site_public_settings;
mod supported_versions;
mod time_unit;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref REQWEST_CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
    static ref MONITORING_API_URL: String = "https://monitoringapi.solaredge.com/".to_string();
}

const URL_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

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
    pub fn new(site_id: &str, api_key: &str) -> Self {
        let site_id = site_id.to_string();
        let api_key = format!("api_key={}", api_key);

        SolaredgeCredentials { site_id, api_key }
    }

    /// See the site ID being used in the credentials.
    pub fn site_id(&self) -> &str {
        &self.site_id
    }
}

/// All Solaredge requests implement this trait since sending the request
/// and getting the response is the same for all requests.
pub trait SendReq<Resp> {
    #[doc(hidden)]
    fn build_url(&self, solaredge: &SolaredgeCredentials) -> String;

    /// Send the request to Solaredge and return the response.
    ///
    /// # Arguments
    ///
    /// * `solaredge` - SolarEdge credentials to use for sending
    ///
    /// # Returns
    /// The SolarEdge response or an error string.
    /// Errors can occur on the request send or when parsing the response.
    fn send(&self, solaredge: &SolaredgeCredentials) -> Result<Resp, Error>
    where
        for<'de> Resp: Deserialize<'de>,
    {
        let url = self.build_url(solaredge);

        let res = REQWEST_CLIENT.get(&url).send()?;

        let parsed = res.json::<Resp>()?;

        Ok(parsed)
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
