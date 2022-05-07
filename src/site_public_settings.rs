//! Module for reporting information about the web pages for the site served by SolarEdge.

use serde::{Deserialize, Serialize};

/// Information about the public web page for the site provided by SolarEdge.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SitePublicSettings {
    /// Optional name given to the web page fro the site.
    pub name: Option<String>,

    /// Is the web page accessible to the public (i.e. no password required)?
    pub is_public: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;

    #[test]
    fn normal_types_unit_test() {
        is_normal::<SitePublicSettings>();
    }
}
