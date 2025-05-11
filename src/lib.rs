//! `vatsim_utils` - a collection of utilities for accessing [VATSIM] data.
//!
//! [VATSIM]: https://vatsim.net/

#![deny(clippy::all, missing_docs)]

#[cfg(feature = "airports")]
pub mod distance;
pub mod errors;
pub mod live_api;
pub mod models;
pub mod rest_api;
