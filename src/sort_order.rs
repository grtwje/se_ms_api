//! Module for specifying sort order of the output from the SolarEdge server monitoring API.

/// Sort order for the sort property
#[derive(Clone, Debug, PartialEq)]
pub enum SortOrder {
    /// Sort ascending
    Asc,

    /// Sort descending
    Desc,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SortOrder::Asc => write!(f, "ASC"),
            SortOrder::Desc => write!(f, "DESC"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_normal;

    #[test]
    fn normal_types_unit_test() {
        is_normal::<SortOrder>();
    }
}
