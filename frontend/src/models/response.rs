//! Contains structs for deserializing responses returned from the API.

use serde::Deserialize;

/// Holds the standard message + status code response sent from the API.
#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    /// The message associated with this response.
    pub message: String,
    /// The status code number associated with this response.
    pub status_code: u16,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            message: "UNKNOWN".to_string(),
            status_code: 500,
        }
    }
}
