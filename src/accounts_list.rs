//! Module for Return the accounts and list of sub-accounts related to the given token.
//! This API accepts parameters for convenient search, sorting and pagination.

use crate::{SendReq, SortOrder, MONITORING_API_URL};
use serde::Deserialize;
use std::collections::HashMap;

/// accounts_list request
#[derive(Clone, Debug, PartialEq)]
pub struct Req {
    size: String,
    start_index: String,
    search_text: String,
    sort_property: String,
    sort_order: String,
}

/// A sorting option for this account list, based on one of its properties.
#[derive(Clone, Debug, PartialEq)]
pub enum SortProperty {
    /// sort by account name
    Name,

    /// sort by account country
    Country,

    /// sort by account city
    City,

    /// sort by account address
    Address,

    /// sort by account zip code
    Zip,

    /// sort by account FAX number
    Fax,

    /// sort by account phone number
    Phone,

    /// sort by account notes
    Notes,
}

impl std::fmt::Display for SortProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SortProperty::Name => write!(f, "name"),
            SortProperty::Country => write!(f, "country"),
            SortProperty::City => write!(f, "city"),
            SortProperty::Address => write!(f, "address"),
            SortProperty::Zip => write!(f, "zip"),
            SortProperty::Fax => write!(f, "fax"),
            SortProperty::Phone => write!(f, "phone"),
            SortProperty::Notes => write!(f, "notes"),
        }
    }
}

/// accounts_list response
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
pub struct Resp {
    /// The sites matching the request.
    pub accounts: Accounts,
}

/// The accounts matching the request.
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
pub struct Accounts {
    /// The count of matching accounts
    pub count: u16,

    /// Array of matching accounts
    pub list: Entries,
}

/// Array of matching accounts
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(transparent)]
pub struct Entries {
    /// Transparent list of accounts
    pub e: Vec<AccountDetails>,
}

/// Detailed information for a single account.
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountDetails {
    /// account ID
    pub id: u32,

    /// account name
    pub name: String,

    /// location associated with account
    pub location: AccountLocation,

    /// the company web site
    pub company_web_site: String,

    /// the account contact person first name and surname
    pub contact_person: String,

    /// the contact person email
    pub email: String,

    /// account phone number
    pub phone_number: String,

    /// account fax number
    pub fax_number: String,

    /// account notes
    pub notes: String,

    /// account parent identifier
    pub parent_id: u32,

    /// Miscellaneous uris associated with the web page for the site.
    pub uris: HashMap<String, String>,
}

/// Location of the account.
#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountLocation {
    /// Country of the SolarEdge inverter.
    pub country: String,

    /// State of the SolarEdge inverter.
    pub state: Option<String>, // seems US specific. should this be Option<String>? probably

    /// City of the SolarEdge inverter.
    pub city: String,

    /// Address line 1 of the SolarEdge inverter.
    pub address: String,

    /// Address line 2 of the SolarEdge inverter.
    pub address2: String,

    /// Zip code 1 of the SolarEdge inverter. Used in UK, in EU?
    pub zip: String, // seems US specific. should this be Option<String>?
}

impl Req {
    /// Create a accounts_list request message that can be sent to SolarEdge.
    ///
    /// # Arguments
    ///
    /// * `size` - The maximum number of accounts returned by this call.
    ///            If you have more than 100 accounts, just request another 100
    ///            accounts with startIndex=100. This will fetch accounts 100-199.
    /// * `start_index` - The first account index to be returned in the results
    /// * `search_text` - Search text for this account
    /// * `sort_property` - A sorting option for this account list, based on
    ///                     one of its properties
    /// * `sort_order` - Sort order for the sort property
    #[must_use]
    pub fn new(
        size: Option<u16>,
        start_index: Option<u16>,
        search_text: Option<String>,
        sort_property: Option<SortProperty>,
        sort_order: Option<SortOrder>,
    ) -> Self {
        let size = match size {
            Some(s) => {
                if s > 0 && s <= 100 {
                    format!("size={s}&")
                } else {
                    String::new()
                }
            }
            None => String::new(),
        };

        let start_index = match start_index {
            Some(si) => format!("startIndex={si}&"),
            None => String::new(),
        };

        let search_text = match search_text {
            Some(st) => format!("searchText={st}&"),
            None => String::new(),
        };

        let sort_property = match sort_property {
            Some(sp) => format!("sortProperty={sp}&"),
            None => String::new(),
        };

        let sort_order = match sort_order {
            Some(so) => format!("sortOrder={so}&"),
            None => String::new(),
        };

        Req {
            size,
            start_index,
            search_text,
            sort_property,
            sort_order,
        }
    }
}

impl SendReq<Resp> for Req {
    fn build_url(&self, _site_id: &str, api_key: &str) -> String {
        format!(
            "{}accounts/list?{}{}{}{}{}{}",
            *MONITORING_API_URL,
            self.size,
            self.start_index,
            self.search_text,
            self.sort_property,
            self.sort_order,
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
        is_normal::<Resp>();
        is_normal::<Accounts>();
        is_normal::<Entries>();
        is_normal::<AccountDetails>();
        is_normal::<AccountLocation>();
    }
}
