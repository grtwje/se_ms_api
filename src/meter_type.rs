//! Module for specifying the meter type in SolarEdge server monitoring API requests and responses.

use serde::Deserialize;

/// Meters supported by SolarEdge.
#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub enum MeterType {
    /// Solar energy produced.
    #[default]
    Production,

    /// Total energy consumed (solar + grid)
    Consumption,

    /// Solar energy consumed.
    SelfConsumption,

    /// Solar energy exported to grid.
    FeedIn,

    /// Energy purchased from grid.
    Purchased,
}

impl std::fmt::Display for MeterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MeterType::Production => write!(f, "Production"),
            MeterType::Consumption => write!(f, "Consumption"),
            MeterType::SelfConsumption => write!(f, "SelfConsumption"),
            MeterType::FeedIn => write!(f, "FeedIn"),
            MeterType::Purchased => write!(f, "Purchased"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;

    #[test]
    fn meter_type_fmt_unit_test() {
        let t = MeterType::Production;
        assert_eq!(format!("{t}"), "Production");
    }

    #[test]
    fn normal_types_unit_test() {
        is_normal::<MeterType>();
    }
}
