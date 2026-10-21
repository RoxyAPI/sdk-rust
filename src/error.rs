use std::{error::Error as StdError, fmt, time::Duration};

/// An HTTP error response from the API.
///
/// Match on [`ApiError::error_code`] for programmatic handling; the `error`
/// text is for people and may change wording. Every code is documented at
/// <https://roxyapi.com/docs/errors>.
#[derive(Debug, serde::Deserialize)]
#[non_exhaustive]
pub struct ApiError {
    /// The HTTP status code.
    #[serde(skip)]
    pub status: u16,
    /// A human-readable message.
    #[serde(default)]
    pub error: String,
    /// The stable machine-readable code, empty when the body carried none.
    #[serde(default)]
    pub code: String,
    /// A link to the documentation of this code.
    pub doc_url: Option<String>,
    /// The `Retry-After` delay sent with the response, if any.
    #[serde(skip)]
    pub retry_after: Option<Duration>,
    /// Every other field of the error body, such as `issues` for a
    /// validation error, `allow` for a wrong method or `suggestions` for an
    /// unknown path.
    #[serde(flatten)]
    pub details: serde_json::Map<String, serde_json::Value>,
}

impl ApiError {
    /// Whether the status is one that [`RetryPolicy`](crate::RetryPolicy)
    /// retries: 429, 500, 502, 503 or 504.
    pub fn is_retryable(&self) -> bool {
        crate::client::retryable_status(self.status)
    }
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
    /// The client configuration or a request argument is invalid. Nothing
    /// was sent.
    Configuration(&'static str),
    /// The request could not be sent or the response could not be read.
    Http(reqwest::Error),
    /// The API answered with an error status.
    Api(Box<ApiError>),
    /// A successful response did not match the expected type. The status and
    /// raw body are kept so callers can inspect an unexpected shape.
    #[non_exhaustive]
    Decode {
        /// The HTTP status code.
        status: u16,
        /// The raw response body.
        body: Vec<u8>,
        /// The JSON decoding error.
        source: serde_json::Error,
    },
    /// The client deadline passed before the call completed.
    Timeout,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(message) => write!(f, "RoxyAPI: {message}"),
            Self::Http(_) => f.write_str("RoxyAPI transport error"),
            Self::Api(error) => error.fmt(f),
            Self::Decode { status, .. } => {
                write!(f, "RoxyAPI {status} response could not be decoded")
            }
            Self::Timeout => f.write_str("RoxyAPI request deadline exceeded"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Http(error) => Some(error),
            Self::Decode { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error.without_url())
    }
}

impl ApiError {
    /// The typed error code. A code this SDK version does not list arrives as
    /// [`ErrorCode::Other`](crate::ErrorCode::Other) with the raw value.
    pub fn error_code(&self) -> crate::ErrorCode {
        crate::ErrorCode::from(self.code.as_str())
    }
}
