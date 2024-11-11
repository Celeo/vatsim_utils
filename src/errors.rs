//! Errors the crate can return for various reasons.
//!
//! Most commonly involved with HTTP API access issues.

use thiserror::Error;

/// Errors that can occur processing data in this crate.
#[derive(Debug, Error)]
pub enum VatsimUtilError {
    /// Error that can be returned by any function that makes HTTP
    /// calls to external resources and receives an error response code.
    #[error("Invalid HTTP status code received: {0}")]
    InvalidStatusCode(u16),
    /// Error for if the underlying `reqwest::Client` threw an error.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Error for being unable to parse JSON from anywhere.
    #[error("Failed to serialize/deserialize JSON")]
    FailedJsonParse(#[from] serde_json::Error),
    /// Error that could theoretically be returned from constructing
    /// an instance of the [`Vatsim`](crate::live_api::Vatsim) struct
    /// via it's `new` function.
    #[error("Could not retrieve a V3 URL from the status page")]
    NoV3Url(),
    /// Error that could theoretically be returned from constructing
    /// an instance of the [`Vatsim`](crate::live_api::Vatsim) struct
    /// via it's `new` function.
    #[error("Could not retrieve a transceivers URL from the status page")]
    NoTransceiversUrl(),
}
