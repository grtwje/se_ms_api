//! Module for handling generic date / value pairs returned by the SolarEdge server monitoring API.

use serde::{Deserialize, Serialize};

/// A date and value pair returned from the monitoring API. The value units are specified by the unit
/// field elsewhere in the response.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct DateValue {
    /// YYYY-mm-dd HH:MM:SS
    pub date: String,

    /// Often an integer, but can be float too. Meaning defined by the context of the response.
    pub value: Option<f32>,
}
