//! Module for holding values for a specified meter type in SolarEdge server monitoring API responses.

pub use crate::date_value::DateValue;
use crate::meter_type::MeterType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// Values for the meter type over a range of dates.
pub struct MeterValue {
    /// The meter type of the associated values.
    pub r#type: MeterType, // had to escape the keyword type to use as a json identifier

    /// Meter readings for each date.
    pub values: Vec<DateValue>,
}
