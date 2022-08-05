//! Wrapper for the VATSIM APIs to get live data from the servers.
//!
//! These functions are [async], as they deal with HTTP requests. You'll need
//! to use an async runtime like [tokio] to run them.
//!
//! See the [struct] docs for usage information.
//!
//! [async]: https://doc.rust-lang.org/std/keyword.async.html
//! [tokio]: https://docs.rs/tokio/latest/tokio/
//! [struct]: Vatsim
//!
//! # Example
//!
//! ```rust,no_run
//! use vatsim_utils::live_api::Vatsim;
//! # async fn _do() {
//! let api = Vatsim::new().await.unwrap();
//! // use `api` ...
//! # }
//! ```

use crate::{
    errors::VatsimUtilError,
    models::{RatingsData, Status, StatusData, TransceiverResponseEntry, V3ResponseData},
};
use log::debug;
use rand::seq::SliceRandom;
use reqwest::{Client, ClientBuilder};

/// Initial VATSIM API requests are made to this endpoint.
const STATUS_URL: &str = "https://status.vatsim.net/status.json";

/// Struct containing access to the VATSIM live APIs - those
/// listed on the [VATSIM Developer Info wiki page].
///
/// [VATSIM Developer Info wiki page]: https://github.com/vatsimnetwork/developer-info/wiki/Data-Feeds
#[derive(Debug)]
pub struct Vatsim {
    client: Client,
    v3_url: String,
    transceivers_url: String,
}

impl Vatsim {
    /// Create a new API struct instance.
    ///
    /// Internally, this function also makes the API call to the status
    /// endpoint to get the endpoint to make later API calls, which
    /// is why this function is also `async`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use vatsim_utils::live_api::Vatsim;
    ///
    /// # async fn _do() {
    /// let api = Vatsim::new().await.unwrap();
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function can fail if the HTTP requests to the VATSIM API status
    /// endpoint fail, as this endpoint is required in order to get and
    /// store URLs to later query for getting data.
    pub async fn new() -> Result<Self, VatsimUtilError> {
        debug!("Creating VATSIM struct instance");
        let client = ClientBuilder::new()
            .user_agent("github.com/celeo/vatsim_utils")
            .build()
            .expect("Invalid HTTP Agent");
        let (v3_url, transceivers_url) = Vatsim::get_endpoint_urls(&client).await?;
        Ok(Self {
            client,
            v3_url,
            transceivers_url,
        })
    }

    /// Get the V3 and transceivers URLs by querying the status endpoint.
    async fn get_endpoint_urls(client: &Client) -> Result<(String, String), VatsimUtilError> {
        debug!("Getting V3 url from status page");
        let response = client.get(STATUS_URL).send().await?;
        if !response.status().is_success() {
            return Err(VatsimUtilError::InvalidStatusCode(
                response.status().as_u16(),
            ));
        }
        let data: StatusData = (response.json::<Status>().await?).data;
        let v3_url = data
            .v3
            .choose(&mut rand::thread_rng())
            .expect("No VATSIM V3 API URLs returned")
            .clone();
        let transceivers_url = data
            .transceivers
            .choose(&mut rand::thread_rng())
            .expect("No VATSIM transceivers API URLs returned")
            .clone();
        debug!("V3 URL: {}, transceiver URL: {}", v3_url, transceivers_url);
        Ok((v3_url, transceivers_url))
    }

    /// Query the stored V3 endpoint.
    ///
    /// This function sorts the pilots and controllers by their
    /// callsigns, alphabetically, before returning.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use vatsim_utils::live_api::Vatsim;
    ///
    /// # async fn _do() {
    /// let api = Vatsim::new().await.unwrap();
    /// let data = api.get_v3_data().await.unwrap();
    /// // use data ...
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function can fail if the HTTP request fails or if the returned
    /// data does not match the schemas of the models passed to the
    /// deserializer.
    ///
    /// # Panics
    ///
    /// Could panic if the callsign `String`s fail `partial_cmp`.
    pub async fn get_v3_data(&self) -> Result<V3ResponseData, VatsimUtilError> {
        debug!("Getting current V3 data");
        let response = self.client.get(&self.v3_url).send().await?;
        if !response.status().is_success() {
            return Err(VatsimUtilError::InvalidStatusCode(
                response.status().as_u16(),
            ));
        }
        let mut data: V3ResponseData = response.json().await?;
        data.pilots
            .sort_by(|a, b| a.callsign.partial_cmp(&b.callsign).unwrap());
        data.controllers
            .sort_by(|a, b| a.callsign.partial_cmp(&b.callsign).unwrap());
        Ok(data)
    }

    /// Get pilot transceiver frequency data.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use vatsim_utils::live_api::Vatsim;
    ///
    /// # async fn _do() {
    /// let api = Vatsim::new().await.unwrap();
    /// let data = api.get_transceivers_data().await.unwrap();
    /// // use data ...
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function can fail if the HTTP request fails or if the returned
    /// data does not match the schemas of the models passed to the
    /// deserializer.
    pub async fn get_transceivers_data(
        &self,
    ) -> Result<Vec<TransceiverResponseEntry>, VatsimUtilError> {
        debug!("Getting current transceivers data");
        let response = self.client.get(&self.transceivers_url).send().await?;
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
    /// use vatsim_utils::live_api::Vatsim;
    ///
    /// # async fn _do() {
    /// let api = Vatsim::new().await.unwrap();
    /// let times = api.get_ratings_times(1234567890).await.unwrap();
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function can fail if the HTTP request fails or if the returned
    /// data does not match the schemas of the models passed to the
    /// deserializer.
    pub async fn get_ratings_times(&self, cid: u64) -> Result<RatingsData, VatsimUtilError> {
        let response = self
            .client
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
}
