//! Contains structs for deserializing responses returned from the API.

use serde::Deserialize;

/// Holds the standard message + status code response sent from the API.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
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

impl Response {
    /// Create a new `Response` with a given message and an HTTP 500 status code.
    pub fn status_500_with_message(message: String) -> Self {
        Self {
            message,
            status_code: 500,
        }
    }
}
