//! Contains all error variants that may be raised.

use thiserror::Error;
use wasm_bindgen::JsValue;

/// Contains all error variants that may be raised.
#[derive(Debug, Error)]
pub enum StaccError {
    /// Something fucked up while setting the background GIF.
    #[error("Could not modify the document.body style: {0:#?}")]
    SetBackgroundError(JsValue),
}

impl From<JsValue> for StaccError {
    fn from(js_value: JsValue) -> Self {
        StaccError::SetBackgroundError(js_value)
    }
}
