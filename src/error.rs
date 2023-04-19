use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    Reqwest(reqwest::Error),
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Serde(err) => err.fmt(f),
            Error::Reqwest(err) => err.fmt(f),
        }
    }
}
