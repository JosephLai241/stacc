//! Contains models for miscellaneous data.

use serde::{Deserialize, Serialize};

/// Contains the Imgur link to the background GIF.
#[derive(Debug, Deserialize, Serialize)]
pub struct BackgroundGIF {
    /// The Imgur link to the background GIF.
    pub link: String,
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
