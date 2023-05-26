//! Contains models for miscellaneous data.

use serde::{Deserialize, Serialize};

/// Contains the Imgur link to the background GIF.
#[derive(Debug, Deserialize, Serialize)]
pub struct BackgroundGIF {
    /// The Imgur link to the background GIF.
    pub link: String,
}
