#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod client;
mod error;
#[rustfmt::skip]
#[allow(clippy::all)]
mod generated;

pub use client::{Builder, RetryPolicy, Roxy};
pub use error::{ApiError, Error};
pub use generated::*;
pub use reqwest;

/// The SDK version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
