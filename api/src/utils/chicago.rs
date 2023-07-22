//! Contains miscellaneous utilities for Chicago-related functionality.

use lazy_static::lazy_static;
use reqwest::Client;
use serde_json::Value;

use crate::{errors::StaccError, models::data::ChicagoMapData};

use super::environment::EnvironmentVariables;

lazy_static! {
    /// A `reqwest` `Client` that is reused for Chicago API requests.
    static ref REQUEST_CLIENT: Client = Client::new();
    /// The API endpoint for the ShotSpotter Alerts data.
    static ref SHOTSPOTTER_ENDPOINT: &'static str = "https://data.cityofchicago.org/resource/3h7q-7mdb.json";
    /// The API endpoint for the Victims of Homicides and Non-Fatal Shootings data.
    static ref VHNFS_ENDPOINT: &'static str = "https://data.cityofchicago.org/resource/gumc-mgzr.json";
}

/// Get data for Victims of Homicides and Non-Fatal Shootings and Shotspotter Alert data from the
/// Chicago APIs.
pub async fn get_vhnfs_shotspotter_data() -> Result<ChicagoMapData, StaccError> {
    let violence_data: Value = serde_json::from_str(
        &REQUEST_CLIENT
            .get(VHNFS_ENDPOINT.to_string())
            .header(
                "X-App-Token",
                EnvironmentVariables::SocrataAppToken.env_var()?,
            )
            .send()
            .await?
            .text()
            .await?,
    )?;
    let shotspotter_data: Value = serde_json::from_str(
        &REQUEST_CLIENT
            .get(SHOTSPOTTER_ENDPOINT.to_string())
            .header(
                "X-App-Token",
                EnvironmentVariables::SocrataAppToken.env_var()?,
            )
            .send()
            .await?
            .text()
            .await?,
    )?;

    let chicago_map_data = ChicagoMapData {
        shotspotter_data,
        violence_data,
    };

    Ok(chicago_map_data)
}
