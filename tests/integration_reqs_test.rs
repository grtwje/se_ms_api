use chrono::NaiveDateTime;

#[macro_use]
extern crate lazy_static;

mod common;

use se_ms_api::{
    CurrentVersionReq, MeterType, SendReq, SiteDetailsReq, SiteEnergyDetailedReq,
    SupportedVersionsReq,
};

#[test]
fn site_energy_detailed_integration_test() {
    let start_ndt = match NaiveDateTime::parse_from_str("2022-01-01 00:00:00", common::TIME_FORMAT)
    {
        Ok(dt) => dt,
        Err(error) => panic!("Error parsing start date: {}", error),
    };

    let end_ndt = match NaiveDateTime::parse_from_str("2022-01-31 00:00:00", common::TIME_FORMAT) {
        Ok(dt) => dt,
        Err(error) => panic!("Error parsing end date: {}", error),
    };

    let req = SiteEnergyDetailedReq::new(
        start_ndt,
        end_ndt,
        None,
        Some(vec![MeterType::SelfConsumption]),
    );

    let resp = req.send(&common::TEST_CREDENTIALS);

    match resp {
        Ok(r) => {
            assert_eq!(r.energy_details.unit, "Wh");
            assert_eq!(r.energy_details.meters.len(), 1);
            assert_eq!(
                r.energy_details.meters[0].meter_type,
                MeterType::SelfConsumption
            );
            assert_eq!(r.energy_details.meters[0].values.len(), 31);

            let mut self_consumption: f32 = 0.0;
            for v in &r.energy_details.meters[0].values {
                if let Some(value) = v.value {
                    self_consumption += value;
                }
            }
            assert!(self_consumption as i32 == 292473);
        }
        Err(e) => {
            panic!("Unexpected SiteEnergyDetailedReq response: {:?}", e);
        }
    };
}

#[test]
fn current_version_integration_test() {
    let req = CurrentVersionReq::new();
    let resp = req.send(&common::TEST_CREDENTIALS);

    match resp {
        Ok(r) => {
            assert_eq!(r.version.release, "1.0.0");
        }
        Err(e) => {
            panic!("Unexpected CurrentVersion response: {:?}", e);
        }
    }
}

#[test]
fn supported_versions_integration_test() {
    let req = SupportedVersionsReq::new();
    let resp = req.send(&common::TEST_CREDENTIALS);

    match resp {
        Ok(r) => {
            assert_eq!(r.supported[0].release, "1.0.0");
        }
        Err(e) => {
            panic!("Unexpected SupportedVersions response: {:?}", e);
        }
    }
}

#[test]
fn site_details_integration_test() {
    let req = SiteDetailsReq::new();
    let resp = req.send(&common::TEST_CREDENTIALS);

    match resp {
        Ok(r) => {
            assert_eq!(r.details.id.to_string(), common::TEST_CREDENTIALS.site_id());
            assert_eq!(r.details.status, "Active");
            assert_eq!(r.details.location.country_code, "US");
            assert_eq!(r.details.primary_module.manufacturer_name, "LG");
            assert!(r.details.uris.contains_key("SITE_IMAGE"));
            assert!(!r.details.public_settings.is_public);
        }
        Err(e) => {
            panic!("Unexpected SiteDetails response: {:?}", e);
        }
    }
}
