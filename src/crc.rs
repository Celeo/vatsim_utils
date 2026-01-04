//! CRC data feed.
//!
//! <https://data-admin.vnas.vatsim.net/docs/#/controller-feed>

use log::debug;
use reqwest::{Client, ClientBuilder};

use crate::{errors::VatsimUtilError, models::DataFeed};

const URL: &str = "https://live.env.vnas.vatsim.net/data-feed/controllers.json";

/// Struct containing access to the CRC data feed.
#[derive(Debug)]
pub struct CRC {
    client: Client,
}

impl Default for CRC {
    fn default() -> Self {
        Self::new()
    }
}

impl CRC {
    /// Create a new API struct instance.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use vatsim_utils::crc::CRC;
    ///
    /// # async fn _do() {
    /// let api = CRC::new();
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Will panic if the HTTP user agent cannot be constructed, which
    /// should never happen.
    pub fn new() -> Self {
        debug!("Creating CRC struct instance");
        let client = ClientBuilder::new()
            .user_agent("github.com/celeo/vatsim_utils")
            .build()
            .expect("Invalid HTTP Agent");
        Self { client }
    }

    /// Get the current CRC data feed data.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use vatsim_utils::crc::CRC;
    ///
    /// # async fn _do() {
    /// let api = CRC::new();
    /// let data = api.get_data().await.unwrap();
    /// // use data ...
    /// # }
    pub async fn get_data(&self) -> Result<DataFeed, VatsimUtilError> {
        debug!("Getting current CRC data feed");
        let response = self.client.get(URL).send().await?;
        if !response.status().is_success() {
            return Err(VatsimUtilError::InvalidStatusCode(
                response.status().as_u16(),
            ));
        }
        let data = response.json().await?;
        Ok(data)
    }
}
