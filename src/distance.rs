//! Utilities for distance calculations and determinations.

use std::f64::consts::PI;

/// Calculate the Haversine Distance between two (lat & long) points.
///
/// Originally from <https://www.movable-type.co.uk/scripts/latlong.html>.
///
/// # Example
///
/// ```rust
/// use vatsim_utils::distance::haversine;
/// let distance = haversine(32.7338, -117.1933, 33.9416, -118.4085);
/// ```
#[must_use]
pub fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371e3;
    let φ1 = (lat1 * PI) / 180_f64;
    let φ2 = (lat2 * PI) / 180_f64;
    #[allow(non_snake_case)]
    let Δφ = ((lat2 - lat1) * PI) / 180_f64;
    #[allow(non_snake_case)]
    let Δλ = ((lon2 - lon1) * PI) / 180_f64;
    let a = f64::sin(Δφ / 2_f64) * f64::sin(Δφ / 2_f64)
        + f64::cos(φ1) * f64::cos(φ2) * f64::sin(Δλ / 2_f64) * f64::sin(Δλ / 2_f64);
    let c = 2_f64 * f64::atan2(f64::sqrt(a), f64::sqrt(1_f64 - a));
    let d = r * c;
    f64::round(d * 0.00054)
}
