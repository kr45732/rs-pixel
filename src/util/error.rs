use serde_json;
use std::fmt;

#[derive(Debug)]
pub enum RsPixelError {
    Client(surf::Error),
    Parse(serde_json::Error),
    Status(u16, String),
    RateLimit(i64),
    Unknown(String),
}

impl From<surf::Error> for RsPixelError {
    fn from(e: surf::Error) -> Self {
        RsPixelError::Client(e)
    }
}

impl From<serde_json::Error> for RsPixelError {
    fn from(e: serde_json::Error) -> Self {
        RsPixelError::Parse(e)
    }
}

impl From<String> for RsPixelError {
    fn from(e: String) -> Self {
        RsPixelError::Unknown(e)
    }
}

impl From<(surf::StatusCode, String)> for RsPixelError {
    fn from(e: (surf::StatusCode, String)) -> Self {
        RsPixelError::Status(e.0.into(), e.1)
    }
}

impl fmt::Display for RsPixelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RsPixelError::Client(ref err) => err.fmt(f),
            RsPixelError::Parse(ref err) => err.fmt(f),
            RsPixelError::Unknown(ref err) => err.fmt(f),
            RsPixelError::Status(ref code, ref err) => write!(f, "{} {}", code, err),
            RsPixelError::RateLimit(ref time_till_reset) => write!(
                f,
                "Reached the rate limit; {} seconds till reset",
                time_till_reset
            ),
        }
    }
}
