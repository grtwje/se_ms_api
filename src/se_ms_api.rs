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
//! * [AccountsListReq] / [AccountsListResp]
//! * [CurrentVersionReq] / [CurrentVersionResp]
//! * [SiteDataPeriodReq] / [SiteDataPeriodResp]
//! * [SiteDetailsReq] / [SiteDetailsResp]
//! * [SiteEnergyReq] / [SiteEnergyResp]
//! * [SiteEnergyDetailedReq] / [SiteEnergyDetailedResp]
//! * [SiteEnvironmentalBenefitsReq] / [SiteEnvironmentalBenefitsResp]
//! * [SiteEquipmentChangeLogReq] / [SiteEquipmentChangeLogResp]
//! * [SiteEquipmentListReq] / [SiteEquipmentListResp]
//! * [SiteGetMetersDataReq] / [SiteGetMetersDataResp]
//! * [SiteGetSensorListReq] / [SiteGetSensorListResp]
//! * [SiteInventoryReq] / [SiteInventoryResp]
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
//! * SiteInverterTechnicalData,
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

pub use accounts_list::{
    AccountDetails, AccountLocation, Accounts, Entries as AccountListEntries,
    Req as AccountsListReq, Resp as AccountsListResp, SortProperty,
};
pub use current_version::{Req as CurrentVersionReq, Resp as CurrentVersionResp, Version};
pub use date_value::DateValue;
pub use error::{Error, Kind};
pub use meter_type::MeterType;
pub use meter_value::MeterValue;
use serde::Deserialize;
pub use site_data_period::{Req as SiteDataPeriodReq, Resp as SiteDataPeriodResp, SiteDataPeriod};
pub use site_details::{Req as SiteDetailsReq, Resp as SiteDetailsResp, SiteDetails};
pub use site_energy::{Energy, Req as SiteEnergyReq, Resp as SiteEnergyResp};
pub use site_energy_detailed::{
    EnergyDetails, Req as SiteEnergyDetailedReq, Resp as SiteEnergyDetailedResp,
};
pub use site_environmental_benefits::{
    EnvBenefits, GasEmissionSaved, Req as SiteEnvironmentalBenefitsReq,
    Resp as SiteEnvironmentalBenefitsResp,
};
pub use site_equipment_change_log::{
    Req as SiteEquipmentChangeLogReq, Resp as SiteEquipmentChangeLogResp,
};
pub use site_equipment_list::{
    Equipment, EquipmentList, Reporters, Req as SiteEquipmentListReq, Resp as SiteEquipmentListResp,
};
pub use site_get_meters_data::{
    Meter as SiteGetMetersDataMeter, MeterEnergyDetails, Req as SiteGetMetersDataReq,
    Resp as SiteGetMetersDataResp,
};
pub use site_get_sensor_list::{
    Gateway as SiteGetSensorListGateway, Gateways, Req as SiteGetSensorListReq,
    Resp as SiteGetSensorListResp, Sensor as SiteGetSensorListSensor, Sensors, SiteSensors,
};
pub use site_inventory::{
    Battery as SiteInventoryBattery, Gateway as SiteInventoryGateway, Inventory, Inverter,
    Meter as SiteInventoryMeter, Req as SiteInventoryReq, Resp as SiteInventoryResp,
    Sensor as SiteInventorySensor,
};
pub use site_list::{Entries as SiteListEntries, Req as SiteListReq, Resp as SiteListResp, Sites};
pub use site_location::SiteLocation;
pub use site_module::SiteModule;
pub use site_overview::{
    CurrentPower, EnergyRevenue, Overview, Req as SiteOverviewReq, Resp as SiteOverviewResp,
};
pub use site_power::{Power, Req as SitePowerReq, Resp as SitePowerResp};
pub use site_power_detailed::{
    PowerDetails, Req as SitePowerDetailedReq, Resp as SitePowerDetailedResp,
};
pub use site_power_flow::{
    Connections, Parameters, Req as SitePowerFlowReq, Resp as SitePowerFlowResp,
    SiteCurrentPowerFlow,
};
pub use site_public_settings::SitePublicSettings;
pub use site_storage_data::{
    Batteries, Battery as SiteStorageDataBattery, Req as SiteStorageDataReq,
    Resp as SiteStorageDataResp, StorageData,
};
pub use site_time_frame_energy::{
    Req as SiteTimeFrameEnergyReq, Resp as SiteTimeFrameEnergyResp, TimeFrameEnergy,
};
pub use sort_order::SortOrder;
pub use supported_versions::{Release, Req as SupportedVersionsReq, Resp as SupportedVersionsResp};
pub use system_units::SystemUnits;
pub use time_unit::TimeUnit;

mod accounts_list;
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
mod site_equipment_change_log;
mod site_equipment_list;
mod site_get_meters_data;
mod site_get_sensor_list;
mod site_inventory;
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
mod sort_order;
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
