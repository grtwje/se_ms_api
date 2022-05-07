//! Module for reporting information about the web pages for the site served by SolarEdge.

use serde::{Deserialize, Serialize};

/// Information about the public web page for the site provided by SolarEdge.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct SitePublicSettings {
    /// Optional name given to the web page fro the site.
    pub name: Option<String>,

    /// Is the web page accessible to the public (i.e. no password required)?
    pub isPublic: bool,
}
