//! Module for querying a list of sites related to the given token, which is the account api_key.
//! This API accepts parameters for convenient search, sort and pagination.

use crate::{SendReq, SiteDetails, SortOrder, MONITORING_API_URL};
use serde::Deserialize;

/// site_list request
#[derive(Clone, Debug, PartialEq)]
pub struct Req {
    size: String,
    start_index: String,
    search_text: String,
    sort_property: String,
    sort_order: String,
    status: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SortProperty {
    /// sort by site name
    Name,

    /// sort by site country
    Country,

    /// sort by site state
    State,

    /// sort by site city
    City,

    /// sort by site address
    Address,

    /// sort by site zip code
    Zip,

    /// sort by site status
    Status,

    /// sort by peak power
    PeakPower,

    /// sort by installation date
    InstallationDate,

    /// sort by amount of alerts
    Amount,

    /// sort by alert severity
    MaxSeverity,

    /// sort by site creation time
    CreationTime,
}

impl std::fmt::Display for SortProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SortProperty::Name => write!(f, "name"),
            SortProperty::Country => write!(f, "country"),
            SortProperty::State => write!(f, "state"),
            SortProperty::City => write!(f, "city"),
            SortProperty::Address => write!(f, "address"),
            SortProperty::Zip => write!(f, "zip"),
            SortProperty::Status => write!(f, "status"),
            SortProperty::PeakPower => write!(f, "peakPower"),
            SortProperty::InstallationDate => write!(f, "installationDate"),
            SortProperty::Amount => write!(f, "amount"),
            SortProperty::MaxSeverity => write!(f, "maxSeverity"),
            SortProperty::CreationTime => write!(f, "creationTime"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    /// Active sites
    Active,

    /// Sites not active yet
    Pending,

    /// Sites that are disabled
    Disabled,

    /// All sites
    All,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Status::Active => write!(f, "Active"),
            Status::Pending => write!(f, "Pending"),
            Status::Disabled => write!(f, "Disabled"),
            Status::All => write!(f, "All"),
        }
    }
}

/// site_list response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
pub struct Resp {
    /// The sites matching the request.
    pub sites: Sites,
}

/// The sites matching the request.
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
pub struct Sites {
    /// The count of matching sites
    pub count: u16,

    /// Array of matching sites
    pub site: Entries,
}

/// Array of matching sites
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(transparent)]
pub struct Entries {
    pub e: Vec<SiteDetails>,
}

impl Req {
    /// Create a site_list request message that can be sent to SolarEdge.
    ///
    /// # Arguments
    ///
    /// * `size` - The maximum number of sites returned by this call.
    ///            If you have more than 100 sites, just request another 100
    ///            sites with startIndex=100. This will fetch sites 100-199.
    /// * `start_index` - The first site index to be returned in the results
    /// * `search_text` - Search text for this site
    /// * `sort_property` - A sorting option for this site list, based on
    ///                     one of its properties
    /// * `sort_order` - Sort order for the sort property
    /// * `status` - Select the sites to be included in the list by their status.
    ///              Default list will include Active and Pending sites.
    #[must_use]
    pub fn new(
        size: Option<u16>,
        start_index: Option<u16>,
        search_text: Option<String>,
        sort_property: Option<SortProperty>,
        sort_order: Option<SortOrder>,
        status: Option<Vec<Status>>,
    ) -> Self {
        let size = match size {
            Some(s) => {
                if s > 0 && s <= 100 {
                    format!("size={}&", s)
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        };

        let start_index = match start_index {
            Some(si) => format!("startIndex={}&", si),
            None => "".to_string(),
        };

        let search_text = match search_text {
            Some(st) => format!("searchText={}&", st),
            None => "".to_string(),
        };

        let sort_property = match sort_property {
            Some(sp) => format!("sortProperty={}&", sp),
            None => "".to_string(),
        };

        let sort_order = match sort_order {
            Some(so) => format!("sortOrder={}&", so),
            None => "".to_string(),
        };

        let status = match status {
            Some(s) => format!(
                "status={}&",
                s.iter()
                    .map(Status::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            None => "".to_string(),
        };

        Req {
            size,
            start_index,
            search_text,
            sort_property,
            sort_order,
            status,
        }
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, _site_id: &str, api_key: &str) -> String {
        format!(
            "{}sites/list?{}{}{}{}{}{}{}",
            *MONITORING_API_URL,
            self.size,
            self.start_index,
            self.search_text,
            self.sort_property,
            self.sort_order,
            self.status,
            api_key,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;

    #[test]
    fn normal_types_unit_test() {
        is_normal::<Req>();
        is_normal::<SortProperty>();
        is_normal::<Status>();
        is_normal::<Resp>();
        is_normal::<Sites>();
        is_normal::<Entries>();
    }
}
