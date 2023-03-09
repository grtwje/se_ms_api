//! Module for the measurement units used with the SolarEdge server monitoring API.

use serde::Deserialize;

/// Time units specified in SolarEdge server monitoring API requests and responses.
/// Specifies the aggregation granularity of the data.
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum SystemUnits {
    /// Imperial measurement units
    Imperial,

    /// Metric measurement units
    Metrics,
}

impl std::fmt::Display for SystemUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SystemUnits::Imperial => write!(f, "Imperial"),
            SystemUnits::Metrics => write!(f, "Metrics"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;

    #[test]
    fn system_units_fmt_unit_test() {
        let t = SystemUnits::Imperial;
        assert_eq!(format!("{t}"), "Imperial");
    }

    #[test]
    fn normal_types_unit_test() {
        is_normal::<SystemUnits>();
    }
}
