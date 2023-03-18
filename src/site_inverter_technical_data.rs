//! Module for specific inverter data for a given time frame.

use crate::{SendReq, MONITORING_API_URL, URL_DATE_TIME_FORMAT};
use serde::Deserialize;

/// site_inverter_technical_data request
#[derive(Clone, Debug, PartialEq)]
pub struct Req {
    serial_number: String,
    start_time: String,
    end_time: String,
}

/// site_inverter_technical_data response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    /// inverter technical data
    pub data: InverterData,
}

/// Inverter data for each telemetry
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InverterData {
    /// Number of telemetries in the list
    pub count: u32,
    /// List of telemetries
    pub telemetries: Telemetries,
}

/// Array of telemetries
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(transparent)]
pub struct Telemetries {
    /// Transparent list of accounts
    pub t: Vec<Telemetry>,
}

/// Data for a single telemetry
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Telemetry {
    /// Date of telemetry collected
    pub date: String,

    /// Total active power
    pub total_active_power: Option<f32>,

    /// DC voltage
    pub dc_voltage: Option<f32>,

    /// Power limit
    pub power_limit: f32,

    /// Total energy
    pub total_energy: f32,

    /// Celsius
    pub temperature: f32,

    /// Operating mode of inverter
    pub inverter_mode: InverterMode,

    /// 0 – On-grid
    /// 1 – Operating in off-grid mode using PV or battery
    /// 2 - Operating in off-grid mode with generator (e.g. diesel) is present
    pub operation_mode: u16,

    /// Data for phase level 1
    #[serde(rename = "L1Data")]
    pub l1_data: LxData,
}

/// Data for a phase level
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LxData {
    /// AC current
    pub ac_current: f32,

    /// AC voltage
    pub ac_voltage: f32,

    /// AC frequency
    pub ac_frequency: f32,

    /// Apparent power
    pub apparent_power: f32,

    /// Active power
    pub active_power: f32,

    /// Reactive power
    pub reactive_power: f32,

    /// cos phi?
    pub cos_phi: f32,
}

/// Inverter operating mode
#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InverterMode {
    /// Off
    #[default]
    Off,

    /// Night mode
    Night,

    /// Pre-production
    WakeUp,
    /// Production
    Production,

    /// Forced power reduction
    ProductionLimit,

    /// Shutdown procedure
    Shutdown,

    /// Error mode
    Error,

    /// Maintenance
    Setup,

    /// Standby mode lock
    LockedStdby,

    /// Fire fighters lock mode
    LockedFireFighters,

    /// Forced shutdown from server
    LockedForceShutdown,

    /// Communication timeout
    LockedCommTimeout,

    /// Inverter self-lock trip
    LockedInvTrip,

    /// Inverter self-lock arc detection
    LockedInvArcDetected,

    /// Inverter lock due to DG mode enable
    #[serde(rename = "LOCKED_DG")]
    LockedDG,

    /// MPPT?
    #[serde(rename = "MPPT")]
    Mppt,

    /// Sleeping
    Sleeping,
}

impl std::fmt::Display for InverterMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            InverterMode::Off => write!(f, "Off"),
            InverterMode::Night => write!(f, "Night"),
            InverterMode::WakeUp => write!(f, "Wake Up"),
            InverterMode::Production => write!(f, "Production"),
            InverterMode::ProductionLimit => write!(f, "Production Limit"),
            InverterMode::Shutdown => write!(f, "Shutdown"),
            InverterMode::Error => write!(f, "Error"),
            InverterMode::Setup => write!(f, "Setup"),
            InverterMode::LockedStdby => write!(f, "Locked Standby"),
            InverterMode::LockedFireFighters => write!(f, "Locked Fire Fighters"),
            InverterMode::LockedForceShutdown => write!(f, "Locked Force Shutdown"),
            InverterMode::LockedCommTimeout => write!(f, "Locked Communication Timeout"),
            InverterMode::LockedInvTrip => write!(f, "Locked Inverter Trip"),
            InverterMode::LockedInvArcDetected => write!(f, "Locked Inverter Arc Detected"),
            InverterMode::LockedDG => write!(f, "Locked DG"),
            InverterMode::Mppt => write!(f, "MPPT"),
            InverterMode::Sleeping => write!(f, "Sleeping"),
        }
    }
}

impl Req {
    /// Create an power details request message that can be sent to SolarEdge.
    ///
    /// # Arguments
    ///
    /// * 'serial_number` - inverter short serial number
    /// * `start_time` - beginning of the time period for the inverter data
    /// * `end_time`   - end of the time period for the inverter data
    #[must_use]
    pub fn new(
        serial_number: &str,
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
    ) -> Self {
        let start_time = format!("startTime={}&", start_time.format(URL_DATE_TIME_FORMAT));

        let end_time = format!("endTime={}&", end_time.format(URL_DATE_TIME_FORMAT));

        Req {
            serial_number: serial_number.to_string(),
            start_time,
            end_time,
        }
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, site_id: &str, api_key: &str) -> String {
        format!(
            "{}equipment/{}/{}/data?{}{}{}",
            *MONITORING_API_URL,
            site_id,
            self.serial_number,
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
    fn site_inverter_technical_data_req_new_unit_test() {
        let dt = "2022-01-01 00:00:00";
        if let Ok(ndt) = NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S") {
            let req = Req::new("foo", ndt, ndt);
            assert_eq!(req.start_time, format!("startTime={dt}&"));
            assert_eq!(req.end_time, format!("endTime={dt}&"));
            assert_eq!(req.serial_number, "foo");
        } else {
            panic!("test failed");
        }
    }

    #[test]
    fn normal_types_unit_test() {
        is_normal::<Req>();
        is_normal::<Resp>();
        is_normal::<InverterData>();
        is_normal::<Telemetries>();
        is_normal::<Telemetry>();
        is_normal::<LxData>();
        is_normal::<InverterMode>();
    }
}
