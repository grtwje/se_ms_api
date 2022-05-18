use std::error;
use std::fmt;

/// An error that can occur in this library.
///
/// Usually errors are when trying to send a request to the SolarEdge server,
/// or when trying to parse the response from the server.
#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

impl Error {
    pub(crate) fn new(kind: Kind) -> Error {
        Error { kind }
    }

    /// Convenience function for getting the kind of error.
    #[must_use]
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

/// The different kinds of errors that can occur.
#[derive(Debug)]
#[non_exhaustive]
pub enum Kind {
    /// An error returned from the reqwest crate.
    ReqwestError(reqwest::Error),

    /// Attempted bulk operation, but bulk list is empty.
    BulkListNone,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            Kind::ReqwestError(_) => "Reqwest error",
            Kind::BulkListNone => "Tried to use empty bulk list.",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Kind::ReqwestError(s) => write!(f, "Reqwest Error: HTTP status-code{}", s),
            Kind::BulkListNone => write!(f, "Empty bulk list error"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::new(Kind::ReqwestError(e))
    }
}
