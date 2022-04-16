use chrono::NaiveDateTime;

mod common;

use se_ms_api::{
    CurrentVersionReq, MeterType, SiteDetailsReq, SiteEnergyDetailedReq, SolaredgeCredentials,
    SupportedVersionsReq,
};

#[test]
fn site_energy_detailed_integration_test() {
    let (site_id, api_key) = common::get_site_id_and_api_key();

    let solar_edge = SolaredgeCredentials::new(&site_id, &api_key);

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

    let resp = req.send(&solar_edge);

    match resp {
        Ok(r) => {
            assert_eq!(r.energyDetails.unit, "Wh");
            assert_eq!(r.energyDetails.meters.len(), 1);
            assert_eq!(r.energyDetails.meters[0].r#type, MeterType::SelfConsumption);
            assert_eq!(r.energyDetails.meters[0].values.len(), 31);

            let mut self_consumption: f32 = 0.0;
            for v in &r.energyDetails.meters[0].values {
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
    let (site_id, api_key) = common::get_site_id_and_api_key();

    let solar_edge = SolaredgeCredentials::new(&site_id, &api_key);

    let req = CurrentVersionReq::new();
    let resp = req.send(&solar_edge);

    match resp {
        Ok(r) => {
            assert_eq!(r.version.release, "1.0.0");
        }
        bad => {
            panic!("Unexpected CurrentVersion response: {:?}", bad);
        }
    }
}

#[test]
fn supported_versions_integration_test() {
    let (site_id, api_key) = common::get_site_id_and_api_key();

    let solar_edge = SolaredgeCredentials::new(&site_id, &api_key);

    let req = SupportedVersionsReq::new();
    let resp = req.send(&solar_edge);

    match resp {
        Ok(r) => {
            assert_eq!(r.supported[0].release, "1.0.0");
        }
        bad => {
            panic!("Unexpected SupportedVersions response: {:?}", bad);
        }
    }
}

#[test]
fn site_details_integration_test() {
    let (site_id, api_key) = common::get_site_id_and_api_key();

    let solar_edge = SolaredgeCredentials::new(&site_id, &api_key);

    let req = SiteDetailsReq::new();
    let resp = req.send(&solar_edge);

    match resp {
        Ok(r) => {
            assert_eq!(r.details.id.to_string(), site_id);
            assert_eq!(r.details.status, "Active");
            assert_eq!(r.details.location.countryCode, "US");
            assert_eq!(r.details.primaryModule.manufacturerName, "LG");
            assert!(r.details.uris.contains_key("SITE_IMAGE"));
            assert!(!r.details.publicSettings.isPublic);
        }
        Err(e) => {
            panic!("Unexpected SiteDetails response: {:?}", e);
        }
    }
}
