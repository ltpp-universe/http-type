use std::{
    error::Error as StdError,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum Error {
    HttpReadError(String),
    InvalidHttpRequest(String),
    Unknown,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HttpReadError(data) => write!(f, "Http read error: {}", data),
            Self::InvalidHttpRequest(data) => write!(f, "Invalid http request: {}", data),
            Self::Unknown => write!(f, "{}", "Unknown"),
        }
    }
}
