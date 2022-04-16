//! Module for reporting information about the web pages for the site served by SolarEdge.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
/// Information about the public web page for the site provided by SolarEdge.
pub struct SitePublicSettings {
    /// Optional name given to the web page fro the site.
    pub name: Option<String>,

    /// Is the web page accessible to the public (i.e. no password required)?
    pub isPublic: bool,
}
