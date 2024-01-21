//! Contains models for miscellaneous data.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Contains the Imgur link to the background GIF.
#[derive(Debug, Deserialize, Serialize)]
pub struct BackgroundGIF {
    /// The Imgur link to the background GIF.
    pub link: String,
}

/// Contains JSON data returned from Chicago map-related APIs.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChicagoMapData {
    /// Data returned from the Shotspotter Alert API endpoint.
    pub shotspotter_data: Value,
    /// Data returned from the Victims of Homicides and Non-Fatal Shootings API endpoint.
    pub violence_data: Value,
}

/// Contains the story for the 404 page.
#[derive(Debug, Deserialize, Serialize)]
pub struct Story {
    /// The body of the story.
    pub story: String,
}

/// Serialize an API response message and status code.
#[derive(Debug, Serialize)]
pub struct Response {
    /// The message associated with this response.
    pub message: String,
    /// The status code number associated with this response.
    pub status_code: u16,
}
