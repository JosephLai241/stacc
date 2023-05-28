//! Contains utilities pertaining to setting the dynamic GIF background.

use gloo_console::error;
use gloo_net::http::Request;
use lazy_static::lazy_static;

use crate::errors::StaccError;

lazy_static! {
    /// The path to the `STATIC.gif` GIF that is set as the default background GIF.
    static ref STATIC_GIF: &'static str = "../../STATIC.gif";
}

/// Make a `GET` request to the API to grab a random background GIF, then set the background image
/// of the site.
pub fn set_background(force_refresh: bool) {
    if force_refresh {
        get_new_background();
    } else {
        match check_background_cookie() {
            Some(gif_link) => {
                if let Err(error) = set_background_gif(&gif_link) {
                    error!(error.to_string());
                }
            }
            None => get_new_background(),
        }
    }
}

/// Check for a `background` cookie to see if the background image is already set. If it is already
/// set, return `Some(<BACKGROUND_IMAGE_LINK>)`.
fn check_background_cookie() -> Option<String> {
    wasm_cookies::get("background").and_then(Result::ok)
}

/// Get a new background GIF link from the API, then set the background after reading the GIF link
/// from the `background` cookie.
fn get_new_background() {
    if let Err(error) = set_background_gif(&STATIC_GIF) {
        error!(error.to_string());
    }

    wasm_bindgen_futures::spawn_local(async move {
        // TODO: REMOVE THE LOCALHOST URL LATER.
        if let Err(error) = Request::get("/api/background").send().await {
            error!("FAILED TO GET A NEW BACKGROUND GIF FROM THE API! DEFAULTING.");
            error!(error.to_string());
        }

        let gif_link = check_background_cookie().unwrap_or(STATIC_GIF.to_string());
        if let Err(error) = set_background_gif(&gif_link) {
            error!(error.to_string());
        }
    })
}

/// Set the background image of the page by modifying the `document.body` element.
fn set_background_gif(gif_link: &str) -> Result<(), StaccError> {
    let style = gloo_utils::body().style();

    style.set_property("background-attachment", "fixed")?;
    style.set_property(
        "background-image",
        &format!("linear-gradient(rgba(0, 0, 0, 0.7), rgba(0, 0, 0, 0.7)), url({gif_link})"),
    )?;
    style.set_property("background-position", "center center")?;
    style.set_property("background-repeat", "no-repeat")?;
    style.set_property("background-size", "cover")?;
    style.set_property("height", "100vh")?;
    style.set_property("width", "100vw")?;
    style.set_property("overflow-x", "hidden")?;

    Ok(())
}
