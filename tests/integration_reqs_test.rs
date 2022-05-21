use chrono::{Local, NaiveDate, NaiveDateTime};

#[macro_use]
extern crate lazy_static;

mod common;

use se_ms_api::{
    CurrentVersionReq, MeterType, SendReq, SiteDataPeriodReq, SiteDetailsReq,
    SiteEnergyDetailedReq, SiteEnergyReq, SitePowerDetailedReq, SitePowerReq,
    SiteTimeFrameEnergyReq, SupportedVersionsReq, TimeUnit,
};

#[test]
fn site_energy_detailed_integration_test() {
    let start_ndt =
        match NaiveDateTime::parse_from_str("2022-01-01 00:00:00", common::DATE_TIME_FORMAT) {
            Ok(dt) => dt,
            Err(error) => panic!("Error parsing start date: {}", error),
        };

    let end_ndt =
        match NaiveDateTime::parse_from_str("2022-01-31 00:00:00", common::DATE_TIME_FORMAT) {
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

#[test]
fn site_power_detailed_integration_test() {
    let start_ndt =
        match NaiveDateTime::parse_from_str("2022-01-01 00:00:00", common::DATE_TIME_FORMAT) {
            Ok(dt) => dt,
            Err(error) => panic!("Error parsing start date: {}", error),
        };

    let end_ndt =
        match NaiveDateTime::parse_from_str("2022-01-31 00:00:00", common::DATE_TIME_FORMAT) {
            Ok(dt) => dt,
            Err(error) => panic!("Error parsing end date: {}", error),
        };

    let req = SitePowerDetailedReq::new(start_ndt, end_ndt, Some(vec![MeterType::Purchased]));

    let resp = req.send(&common::TEST_CREDENTIALS);

    match resp {
        Ok(r) => {
            assert_eq!(r.power_details.time_unit, TimeUnit::QuarterOfAnHour);
            assert_eq!(r.power_details.unit, "W");
            assert_eq!(r.power_details.meters.len(), 1);
            assert_eq!(r.power_details.meters[0].meter_type, MeterType::Purchased);
            assert_eq!(r.power_details.meters[0].values.len(), 2880);

            let mut self_consumption: f32 = 0.0;
            for v in &r.power_details.meters[0].values {
                if let Some(value) = v.value {
                    self_consumption += value;
                }
            }
            assert!(self_consumption == 2277237.5);
        }
        Err(e) => {
            panic!("Unexpected SitePowerDetailedReq response: {:?}", e);
        }
    };
}

#[test]
fn site_data_period_integration_test() {
    let req = SiteDataPeriodReq::new();
    let resp = req.send(&common::TEST_CREDENTIALS);

    match resp {
        Ok(r) => {
            if let Some(sd) = r.data_period.start_date {
                assert_eq!(sd, "2018-02-08");
            } else {
                panic!("SiteDataPeriod start date is none.")
            }
            if let Some(ed) = r.data_period.end_date {
                let mut today = Local::today().to_string();
                today.truncate(10);
                assert_eq!(ed, today);
            } else {
                panic!("SiteDataPeriod end date is none.")
            }
        }
        Err(e) => {
            panic!("Unexpected SiteDataPeriod response: {:?}", e);
        }
    }
}

#[test]
fn site_energy_integration_test() {
    let start_date = match NaiveDate::parse_from_str("2022-01-01", common::DATE_FORMAT) {
        Ok(dt) => dt,
        Err(error) => panic!("Error parsing start date: {}", error),
    };

    let end_date = match NaiveDate::parse_from_str("2022-01-02", common::DATE_FORMAT) {
        Ok(dt) => dt,
        Err(error) => panic!("Error parsing end date: {}", error),
    };

    let req = SiteEnergyReq::new(start_date, end_date, None);
    let resp = req.send(&common::TEST_CREDENTIALS);

    match resp {
        Ok(r) => {
            assert_eq!(r.energy.time_unit, TimeUnit::Day);
            assert_eq!(r.energy.unit, "Wh");
            assert_eq!(r.energy.values.len(), 2);
            assert_eq!(r.energy.values[0].date, "2022-01-01 00:00:00");
            assert_eq!(r.energy.values[1].date, "2022-01-02 00:00:00");

            if let Some(v) = r.energy.values[0].value {
                assert_eq!(v, 12926.0);
            } else {
                panic!("Missing value.");
            }
            if let Some(v) = r.energy.values[1].value {
                assert_eq!(v, 4419.0);
            } else {
                panic!("Missing value.");
            }
        }
        Err(e) => panic!("Unexpected SiteEnergy response: {:?}", e),
    }
}

#[test]
fn site_time_frame_energy_integration_test() {
    let start_date = match NaiveDate::parse_from_str("2022-01-01", common::DATE_FORMAT) {
        Ok(dt) => dt,
        Err(error) => panic!("Error parsing start date: {}", error),
    };

    let end_date = match NaiveDate::parse_from_str("2022-01-02", common::DATE_FORMAT) {
        Ok(dt) => dt,
        Err(error) => panic!("Error parsing end date: {}", error),
    };

    let req = SiteTimeFrameEnergyReq::new(start_date, end_date);
    let resp = req.send(&common::TEST_CREDENTIALS);

    match resp {
        Ok(r) => {
            assert_eq!(r.time_frame_energy.unit, "Wh");
            assert_eq!(r.time_frame_energy.energy, 12896.0);
        }
        Err(e) => panic!("Unexpected SiteTimeFrameEnergy response: {:?}", e),
    }
}

#[test]
fn site_power_integration_test() {
    let start_date =
        match NaiveDateTime::parse_from_str("2022-01-01 12:00:00", common::DATE_TIME_FORMAT) {
            Ok(dt) => dt,
            Err(error) => panic!("Error parsing start date: {}", error),
        };

    let end_date =
        match NaiveDateTime::parse_from_str("2022-01-01 13:00:00", common::DATE_TIME_FORMAT) {
            Ok(dt) => dt,
            Err(error) => panic!("Error parsing end date: {}", error),
        };

    let req = SitePowerReq::new(start_date, end_date);
    let resp = req.send(&common::TEST_CREDENTIALS);

    match resp {
        Ok(r) => {
            assert_eq!(r.power.time_unit, TimeUnit::QuarterOfAnHour);
            assert_eq!(r.power.unit, "W");
            assert_eq!(r.power.values.len(), 4);
            assert_eq!(r.power.values[0].date, "2022-01-01 12:00:00");
            assert_eq!(r.power.values[1].date, "2022-01-01 12:15:00");
            assert_eq!(r.power.values[2].date, "2022-01-01 12:30:00");
            assert_eq!(r.power.values[3].date, "2022-01-01 12:45:00");

            if let Some(v) = r.power.values[0].value {
                assert_eq!(v, 2013.872);
            } else {
                panic!("Missing value.");
            }
            if let Some(v) = r.power.values[3].value {
                assert_eq!(v, 1670.7087);
            } else {
                panic!("Missing value.");
            }
        }
        Err(e) => panic!("Unexpected SitePower response: {:?}", e),
    }
}
