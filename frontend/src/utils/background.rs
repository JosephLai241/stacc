//! Contains utilities pertaining to setting the dynamic GIF background.

use gloo_net::http::Request;

/// Make a `GET` request to the API to grab a random background GIF, then set the background image
/// of the site.
pub fn set_background() {
    unimplemented!()
}

/// Check for a `background` cookie to see if the background image is already set. If it is already
/// set, return `Some(<BACKGROUND_IMAGE_LINK>)`.
fn check_background_cookie() -> Option<String> {
    wasm_cookies::get("background").and_then(Result::ok)
}
