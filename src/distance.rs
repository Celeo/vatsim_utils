//! Utilities for distance calculations and airport locations.
//!
//! This module includes a long collection of airport identifiers and lat/long
//! values, available in a list [`AIRPORTS`] and `HashMap` [`AIRPORTS_MAP`].
//!
//! The included [haversine] function can be used to get
//! the distance between two points' lat/long, wether those points
//! be airports, pilots via the [`get_v3_data`] function, or a combination.
//!
//! [`get_v3_data`]: crate::live_api::Vatsim::get_v3_data

use std::{collections::HashMap, f64::consts::PI, sync::LazyLock};

/// Raw airport data from the CSV file.
const AIRPORT_DATA: &str = include_str!("airport_data.csv");

/// Static airport data. Includes latitude and longitude.
///
/// Primarily for use in determining pilot distance to airport
/// via use of the `haversine` function in this module.
#[derive(Debug, Clone, Copy)]
pub struct Airport {
    /// Airport identifier
    pub identifier: &'static str,
    /// Airport decimal latitude
    pub latitude: f64,
    /// Airport decimal longitude
    pub longitude: f64,
}

/// List of included airport identifiers and locations.
///
/// For the entire list, view the [`airport_data.csv`] file
/// in the repo.
///
/// [`airport_data.csv`]: https://github.com/Celeo/vatsim_utils/blob/master/src/airport_data.csv
///
/// # Example
///
/// ```rust
/// use vatsim_utils::distance::AIRPORTS;
///
/// println!("{}", AIRPORTS.get(0).unwrap().identifier);
/// ```
pub static AIRPORTS: LazyLock<Vec<Airport>> = LazyLock::new(|| {
    AIRPORT_DATA
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<_> = line.split(',').collect();
            Airport {
                identifier: parts.first().unwrap(),
                latitude: parts.get(1).unwrap().parse().unwrap(),
                longitude: parts.get(2).unwrap().parse().unwrap(),
            }
        })
        .collect()
});

/// Map of included airport identifiers and locations.
///
/// For the entire list, view the [`airport_data.csv`] file
/// in the repo.
///
/// [`airport_data.csv`]: https://github.com/Celeo/vatsim_utils/blob/master/src/airport_data.csv
///
/// # Example
///
/// ```rust
/// use vatsim_utils::distance::AIRPORTS_MAP;
///
/// println!("{}", AIRPORTS_MAP.get("KSAN").unwrap().identifier);
/// ```
pub static AIRPORTS_MAP: LazyLock<HashMap<&'static str, Airport>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    AIRPORT_DATA
        .split('\n')
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let parts: Vec<_> = line.split(',').collect();
            let airport = Airport {
                identifier: parts.first().unwrap(),
                latitude: parts.get(1).unwrap().parse().unwrap(),
                longitude: parts.get(2).unwrap().parse().unwrap(),
            };
            let _ = m.insert(*parts.first().unwrap(), airport);
        });
    m
});

/// Calculate the Haversine Distance between two (lat & long) points.
///
/// Originally from <https://www.movable-type.co.uk/scripts/latlong.html>.
///
/// # Examples
///
/// With hardcoded values:
///
/// ```rust
/// use vatsim_utils::distance::haversine;
///
/// let distance = haversine(32.7338, -117.1933, 33.9416, -118.4085);
///
/// assert_eq!(distance.round() as i64, 95);
/// ```
///
/// Using lookups:
///
/// ```rust
/// use vatsim_utils::distance::{AIRPORTS, haversine};
///
/// let ksan = AIRPORTS.iter().find(|airport| airport.identifier == "KSAN").unwrap();
/// let klax = AIRPORTS.iter().find(|airport| airport.identifier == "KLAX").unwrap();
/// let distance = haversine(ksan.latitude, ksan.longitude, klax.latitude, klax.longitude);
///
/// assert_eq!(distance.round() as i64, 95);
/// ```
#[allow(clippy::must_use_candidate)]
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
