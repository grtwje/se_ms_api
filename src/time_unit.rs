//! Module for handling units of time used by the SolarEdge server monitoring API.

#[allow(non_camel_case_types)]
/// Time units specified in SolarEdge server monitoring API requests and responses.
/// Specifies the aggregation granularity of the data.
pub enum TimeUnit {
    /// 15 minutes
    QUARTER_OF_AN_HOUR,

    /// 60 minutes
    HOUR,

    /// 24 hours
    DAY,

    /// 7 days
    WEEK,

    /// Calendar month
    MONTH,

    /// Calendar year
    YEAR,
}

impl std::fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TimeUnit::QUARTER_OF_AN_HOUR => write!(f, "QUARTER_OF_AN_HOUR"),
            TimeUnit::HOUR => write!(f, "HOUR"),
            TimeUnit::DAY => write!(f, "DAY"),
            TimeUnit::WEEK => write!(f, "WEEK"),
            TimeUnit::MONTH => write!(f, "MONTH"),
            TimeUnit::YEAR => write!(f, "YEAR"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_unit_fmt_unit_test() {
        let t = TimeUnit::YEAR;
        assert_eq!(format!("{}", t), "YEAR");
    }
}
