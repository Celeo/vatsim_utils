//! VATSIM's public & authenticated REST APIs on [api.vatsim.net] along
//! with some other pages.
//!
//! These functions are not grouped into a struct, as the URLs that
//! they call are static - not dependent on a preceding call - unlike
//! those used to get live data from the network.
//!
//! [api.vatsim.net]: https://api.vatsim.net/

use crate::{
    errors::VatsimUtilError,
    models::{
        AtcSessionEntry, ConnectionEntry, Facility, PaginatedResponse, RatingsTimeData, Region,
        RestFlightPlans, UserRatingsSimple,
    },
};
use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder, Method};
use std::fmt::Write;

/// HTTP client.
static CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        .user_agent("github.com/celeo/vatsim_utils")
        .build()
        .expect("Invalid HTTP Agent")
});

/// Get the URL for viewing a user's stats on stats.vatsim.net.
///
/// This function just returns the URL; the caller is responsible
/// for either giving it to a user or opening it in a browser.
///
/// # Example
///
/// ```rust
/// use vatsim_utils::rest_api::stats_url;
///
/// let url = stats_url(1234567890);
///
/// assert_eq!(&url, "https://stats.vatsim.net/stats/1234567890");
/// ```
#[must_use]
pub fn stats_url(cid: u64) -> String {
    format!("https://stats.vatsim.net/stats/{}", cid)
}

/// Get a simple view of a user's ratings on the network.
///
/// # Example
///
/// ```rust,no_run
/// use vatsim_utils::rest_api::user_ratings;
///
/// # async fn _do() {
/// let info = user_ratings(1234567890).await.unwrap();
/// # }
/// ```
///
/// # Errors
///
/// This function can fail if the HTTP request fails or if the returned
/// data does not match the schemas of the models passed to the
/// deserializer.
pub async fn user_ratings(cid: u64) -> Result<UserRatingsSimple, VatsimUtilError> {
    let response = CLIENT
        .get(format!("https://api.vatsim.net/api/ratings/{}/", cid))
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(VatsimUtilError::InvalidStatusCode(
            response.status().as_u16(),
        ));
    }
    let data = response.json().await?;
    Ok(data)
}

/// Get the amount of time the user has spent as various positions on the network.
///
/// # Example
///
/// ```rust,no_run
/// use vatsim_utils::rest_api::get_ratings_times;
///
/// # async fn _do() {
/// let times = get_ratings_times(1234567890).await.unwrap();
/// # }
/// ```
///
/// # Errors
///
/// This function can fail if the HTTP request fails or if the returned
/// data does not match the schemas of the models passed to the
/// deserializer.
pub async fn get_ratings_times(cid: u64) -> Result<RatingsTimeData, VatsimUtilError> {
    let response = CLIENT
        .get(format!(
            "https://api.vatsim.net/api/ratings/{}/rating_times",
            cid
        ))
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(VatsimUtilError::InvalidStatusCode(
            response.status().as_u16(),
        ));
    }
    let data = response.json().await?;
    Ok(data)
}

/// Get a list of all the user's previous connections.
///
/// A page number can optionally be specified.
///
/// # Example
///
/// ```rust,no_run
/// use vatsim_utils::rest_api::get_connections;
///
/// # async fn _do() {
/// let connections = get_connections(1234567890, None).await.unwrap();
/// // or ...
/// let connections = get_connections(1234567890, Some(3)).await.unwrap();
/// # }
/// ```
///
/// # Errors
///
/// This function can fail if the HTTP request fails or if the returned
/// data does not match the schemas of the models passed to the
/// deserializer.
pub async fn get_connections(
    cid: u64,
    page: Option<u64>,
) -> Result<PaginatedResponse<ConnectionEntry>, VatsimUtilError> {
    let mut url = format!("https://api.vatsim.net/api/ratings/{}/connections", cid);
    if let Some(p) = page {
        let _ = write!(url, "?page={}", p);
    }
    let response = CLIENT.get(url).send().await?;
    if !response.status().is_success() {
        return Err(VatsimUtilError::InvalidStatusCode(
            response.status().as_u16(),
        ));
    }
    let data = response.json().await?;
    Ok(data)
}

/// Get a user's ATC sessions.
///
/// A page number can optionally be specified.
///
/// A position specifier can optionally be specified. For information on what can be
/// included, see [this post].
///
/// [this post]: https://forums.vatsim.net/topic/20-info-on-vatsim-api/#comment-164075
///
/// # Example
///
/// ```rust,no_run
/// use vatsim_utils::rest_api::get_atc_sessions;
///
/// # async fn _do() {
/// let connections = get_atc_sessions(1234567890, None, None, None, None).await.unwrap();
/// // or ...
/// let connections = get_atc_sessions(
///     1234567890,
///     Some(2),
///     Some("SAN_TWR"),
///     Some("2020-01-02"),
///     None,
/// )
/// .await
/// .unwrap();
/// # }
/// ```
///
/// # Errors
///
/// This function can fail if the HTTP request fails or if the returned
/// data does not match the schemas of the models passed to the
/// deserializer.
pub async fn get_atc_sessions(
    cid: u64,
    page: Option<u64>,
    specifier: Option<&str>,
    start: Option<&str>,
    date: Option<&str>,
) -> Result<PaginatedResponse<AtcSessionEntry>, VatsimUtilError> {
    let mut url = format!("https://api.vatsim.net/api/ratings/{}/atcsessions/", cid);
    if let Some(spec) = specifier {
        url += spec;
    }
    let mut req = CLIENT.request(Method::GET, url);
    if let Some(p) = page {
        req = req.query(&[("page", p.to_string().as_str())]);
    }
    if let Some(s) = start {
        req = req.query(&[("start", s)]);
    }
    if let Some(d) = date {
        req = req.query(&[("date", d)]);
    }
    let response = req.send().await?;
    if !response.status().is_success() {
        return Err(VatsimUtilError::InvalidStatusCode(
            response.status().as_u16(),
        ));
    }
    let response_data = response.json().await?;
    Ok(response_data)
}

/// Get a list of all the user's previous flight plans.
///
/// Note that the structs returned by this function contain different
/// fields from flight plans returned by the V3 live API.
///
/// A page number can optionally be specified.
///
/// # Example
///
/// ```rust,no_run
/// use vatsim_utils::rest_api::get_flight_plans;
///
/// # async fn _do() {
/// let connections = get_flight_plans(1234567890, None).await.unwrap();
/// // or ...
/// let connections = get_flight_plans(1234567890, Some(3)).await.unwrap();
/// # }
/// ```
///
/// # Errors
///
/// This function can fail if the HTTP request fails or if the returned
/// data does not match the schemas of the models passed to the
/// deserializer.
pub async fn get_flight_plans(
    cid: u64,
    page: Option<u64>,
) -> Result<PaginatedResponse<RestFlightPlans>, VatsimUtilError> {
    let mut url = format!("https://api.vatsim.net/api/ratings/{}/flight_plans", cid);
    if let Some(p) = page {
        url += &format!("?page={}", p);
    }
    let response = CLIENT.get(url).send().await?;
    if !response.status().is_success() {
        return Err(VatsimUtilError::InvalidStatusCode(
            response.status().as_u16(),
        ));
    }
    let data = response.json().await?;
    Ok(data)
}

/// Get the VATSIM regions.
///
/// # Example
///
/// ```rust,no_run
/// use vatsim_utils::rest_api::get_regions;
///
/// # async fn _do() {
/// let regions = get_regions().await.unwrap();
/// # }
/// ```
///
/// # Errors
///
/// This function can fail if the HTTP request fails or if the returned
/// data does not match the schemas of the models passed to the
/// deserializer.
pub async fn get_regions() -> Result<Vec<Region>, VatsimUtilError> {
    let response = CLIENT
        .get("https://api.vatsim.net/api/regions/")
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(VatsimUtilError::InvalidStatusCode(
            response.status().as_u16(),
        ));
    }
    let data = response.json().await?;
    Ok(data)
}

/// Get facilities currently staffed by ATC.
///
/// # Example
///
/// ```rust,no_run
/// use vatsim_utils::rest_api::get_online_facilities;
///
/// # async fn _do() {
/// let facilities = get_online_facilities().await.unwrap();
/// # }
/// ```
///
/// # Errors
///
/// This function can fail if the HTTP request fails or if the returned
/// data does not match the schemas of the models passed to the
/// deserializer.
pub async fn get_online_facilities() -> Result<Vec<Facility>, VatsimUtilError> {
    let response = CLIENT
        .get("https://api.vatsim.net/api/facilities/")
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(VatsimUtilError::InvalidStatusCode(
            response.status().as_u16(),
        ));
    }
    let data = response.json().await?;
    Ok(data)
}

/// Get a facility's historical staffing data.
///
/// A page number and start and end dates are optional.
///
/// # Example
///
/// ```rust,no_run
/// use vatsim_utils::rest_api::get_facility_history;
///
/// # async fn _do() {
/// let connections = get_facility_history("SAN_TWR", None, None, None).await.unwrap();
/// // or ...
/// let connections = get_facility_history(
///     "SAN_TWR",
///     Some(2),
///     Some("2022-02-01"),
///     None
/// )
/// .await
/// .unwrap();
/// # }
/// ```
///
/// # Errors
///
/// This function can fail if the HTTP request fails or if the returned
/// data does not match the schemas of the models passed to the
/// deserializer.
pub async fn get_facility_history(
    specifier: &str,
    page: Option<u64>,
    start: Option<&str>,
    date: Option<&str>,
) -> Result<PaginatedResponse<AtcSessionEntry>, VatsimUtilError> {
    let mut req = CLIENT.request(
        Method::GET,
        format!("https://api.vatsim.net/api/facilities/{}", specifier),
    );
    if let Some(p) = page {
        req = req.query(&[("page", p.to_string().as_str())]);
    }
    if let Some(s) = start {
        req = req.query(&[("start", s)]);
    }
    if let Some(d) = date {
        req = req.query(&[("date", d)]);
    }
    let response = req.send().await?;
    if !response.status().is_success() {
        return Err(VatsimUtilError::InvalidStatusCode(
            response.status().as_u16(),
        ));
    }
    let response_data = response.json().await?;
    Ok(response_data)
}
