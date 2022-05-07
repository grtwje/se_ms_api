//! Module for holding values for a specified meter type in SolarEdge server monitoring API responses.

pub use crate::date_value::DateValue;
use crate::meter_type::MeterType;
use serde::{Deserialize, Serialize};

/// Values for the meter type over a range of dates.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct MeterValue {
    /// The meter type of the associated values.
    #[serde(rename = "type")]
    pub meter_type: MeterType,

    /// Meter readings for each date.
    pub values: Vec<DateValue>,
}
