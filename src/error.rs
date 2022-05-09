use std::error;
use std::fmt;

/// An error that can occur in this library.
///
/// Usually errors are when trying to send a request to the SolarEdge server,
/// or when trying to parse the response from the server.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }

    /// Convenience function for getting the kind of error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

/// The different kinds of errors that can occur.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// An error returned from the reqwest crate.
    ReqwestError(reqwest::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::ReqwestError(_) => "Reqwest error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::ReqwestError(s) => write!(f, "Reqwest Error: HTTP status-code{}", s),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::new(ErrorKind::ReqwestError(e))
    }
}
