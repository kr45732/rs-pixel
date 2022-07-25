use serde_json;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Client(surf::Error),
    Parse(serde_json::Error),
    Status(u16, String),
    RateLimit(i64),
    Unknown(String),
}

impl From<surf::Error> for Error {
    fn from(e: surf::Error) -> Self {
        Error::Client(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Parse(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Unknown(e)
    }
}

impl From<(surf::StatusCode, String)> for Error {
    fn from(e: (surf::StatusCode, String)) -> Self {
        Error::Status(e.0.into(), e.1)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Client(ref err) => err.fmt(f),
            Error::Parse(ref err) => err.fmt(f),
            Error::Unknown(ref err) => err.fmt(f),
            Error::Status(ref code, ref err) => write!(f, "{} {}", code, err),
            Error::RateLimit(ref time_till_reset) => write!(
                f,
                "Reached the rate limit; {} seconds till reset",
                time_till_reset
            ),
        }
    }
}
