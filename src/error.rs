use std::{error::Error as StdError, fmt, time::Duration};

/// An HTTP error response. Match `code` for programmatic handling.
#[derive(Debug, serde::Deserialize)]
pub struct ApiError {
    #[serde(skip)]
    pub status: u16,
    #[serde(default)]
    pub error: String,
    #[serde(default)]
    pub code: String,
    pub doc_url: Option<String>,
    #[serde(skip)]
    pub retry_after: Option<Duration>,
    #[serde(flatten)]
    pub details: serde_json::Map<String, serde_json::Value>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RoxyAPI {} {}: {}", self.status, self.code, self.error)
    }
}

impl StdError for ApiError {}

/// A configuration, transport, API, response decoding, or deadline error.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Configuration(&'static str),
    Http(reqwest::Error),
    Api(Box<ApiError>),
    Decode(serde_json::Error),
    Timeout,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(message) => write!(f, "RoxyAPI: {message}"),
            Self::Http(error) => write!(f, "RoxyAPI transport: {error}"),
            Self::Api(error) => error.fmt(f),
            Self::Decode(error) => write!(f, "RoxyAPI response: {error}"),
            Self::Timeout => f.write_str("RoxyAPI request deadline exceeded"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Http(error) => Some(error),
            Self::Api(error) => Some(error.as_ref()),
            Self::Decode(error) => Some(error),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error.without_url())
    }
}
