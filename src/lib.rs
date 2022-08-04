//! `vatsim_utils` - a collection of utilities for accessing [VATSIM] data.
//!
//! [VATSIM]: https://vatsim.net/

#![deny(
    clippy::all,
    clippy::pedantic,
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

pub mod distance;
pub mod errors;
pub mod live_api;
pub mod models;
