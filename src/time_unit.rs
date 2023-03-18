//! Module for handling units of time used by the SolarEdge server monitoring API.

use serde::Deserialize;

/// Time units specified in SolarEdge server monitoring API requests and responses.
/// Specifies the aggregation granularity of the data.
#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeUnit {
    /// 15 minutes
    QuarterOfAnHour,

    /// 60 minutes
    Hour,

    /// 24 hours
    #[default]
    Day,

    /// 7 days
    Week,

    /// Calendar month
    Month,

    /// Calendar year
    Year,
}

impl std::fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TimeUnit::QuarterOfAnHour => write!(f, "QUARTER_OF_AN_HOUR"),
            TimeUnit::Hour => write!(f, "HOUR"),
            TimeUnit::Day => write!(f, "DAY"),
            TimeUnit::Week => write!(f, "WEEK"),
            TimeUnit::Month => write!(f, "MONTH"),
            TimeUnit::Year => write!(f, "YEAR"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;

    #[test]
    fn time_unit_fmt_unit_test() {
        let t = TimeUnit::Year;
        assert_eq!(format!("{t}"), "YEAR");
    }

    #[test]
    fn normal_types_unit_test() {
        is_normal::<TimeUnit>();
    }
}
