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
//! use se_ms_api::{SiteDetailsReq, SolaredgeCredentials, Request, Response};
//!
//! let site_id = "my_site_id";
//! let api_key = "my_api_key";
//!
//! let cred = SolaredgeCredentials::create(&site_id, &api_key); // (1)
//! let req = Request::SiteDetails(SiteDetailsReq::new());       // (2)
//!
//! match req.send(&cred) {                                      // (3)
//!    Ok(resp) => {
//!        if let Response::SiteDetails(r) = resp {              // (4)
//!            println!("My site's status is {}.", r.details.status);
//!        }
//!    }
//!    Err(e) => {
//!        panic!("Unexpected SiteDetails response: {:?}", e);
//!    }
//!}
//! ```

//#![warn(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
//#![warn(missing_docs)]

pub mod site_details;
pub use site_details::{SiteDetailsReq, SiteDetailsResp};
pub mod site_energy_detailed;
pub use site_energy_detailed::{SiteEnergyDetailedReq, SiteEnergyDetailedResp};
pub mod current_version;
pub use current_version::{CurrentVersionReq, CurrentVersionResp};
pub mod supported_versions;
pub use supported_versions::{SupportedVersionsReq, SupportedVersionsResp};
pub mod date_value;
pub mod meter_type;
pub use meter_type::MeterType;
pub mod meter_value;
pub mod site_location;
pub mod site_module;
pub mod site_public_settings;
pub mod time_unit;

const URL_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// Struct for accessing SolarEdge's monitoring server for a given site and api key.
///
/// Used as the parameter for the send() function of all of the possible requests.
pub struct SolaredgeCredentials {
    url_start: String,
    site_id: String,
    url_end: String,
}

impl SolaredgeCredentials {
    const MONITORING_API_URL: &'static str = "https://monitoringapi.solaredge.com/";

    /// Create a Solaredge destination for the requests from the given site id and api_key.
    pub fn create(site_id: &str, api_key: &str) -> Self {
        let url_start = SolaredgeCredentials::MONITORING_API_URL.to_string();
        let site_id = site_id.to_string();
        let url_end = format!("api_key={}", api_key);

        SolaredgeCredentials {
            url_start,
            site_id,
            url_end,
        }
    }

    /// See the site ID bing used in the credentials.
    pub fn site_id(&self) -> &str {
        &self.site_id
    }
}

pub enum Request {
    CurrentVersion(CurrentVersionReq),
    SiteDetails(SiteDetailsReq),
    SiteEnergyDetailed(SiteEnergyDetailedReq),
    SupportedVersions(SupportedVersionsReq),
}

pub enum Response {
    CurrentVersion(CurrentVersionResp),
    SiteDetails(Box<SiteDetailsResp>),
    SiteEnergyDetailed(SiteEnergyDetailedResp),
    SupportedVersions(SupportedVersionsResp),
}

impl Request {
    pub fn send(&self, cred: &SolaredgeCredentials) -> Result<Response, String> {
        match self {
            Request::CurrentVersion(r) => r.send(cred),
            Request::SiteDetails(r) => r.send(cred),
            Request::SiteEnergyDetailed(r) => r.send(cred),
            Request::SupportedVersions(r) => r.send(cred),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solaredge_credentials_unit_test() {
        let se = SolaredgeCredentials::create("id", "key");
        assert_eq!(se.url_start, SolaredgeCredentials::MONITORING_API_URL);
        assert_eq!(se.site_id, "id");
        assert_eq!(se.site_id(), "id");
        assert_eq!(se.url_end, "api_key=key");
    }
}
