//! Contains all error variants that may be raised.

use thiserror::Error;
use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlTableElement};

/// Contains all error variants that may be raised.
#[derive(Debug, Error)]
pub enum StaccError {
    /// Something fucked up while interacting with `Element`s.
    #[error("Element error: {0:#?}")]
    ElementError(Element),

    /// Something fucked up while interacting with `HtmlTableElement`s.
    #[error("HtmlTableElement error: {0:#?}")]
    HtmlTableElementError(HtmlTableElement),

    /// Something fucked up while interacting with `JsValue`s.
    #[error("JsValue error: {0:#?}")]
    JsValueError(JsValue),
}

impl From<Element> for StaccError {
    fn from(element: Element) -> Self {
        StaccError::ElementError(element)
    }
}

impl From<HtmlTableElement> for StaccError {
    fn from(html_table_element: HtmlTableElement) -> Self {
        StaccError::HtmlTableElementError(html_table_element)
    }
}

impl From<JsValue> for StaccError {
    fn from(js_value: JsValue) -> Self {
        StaccError::JsValueError(js_value)
    }
}
