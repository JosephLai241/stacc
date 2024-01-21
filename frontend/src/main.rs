//! `stacc` -- A Rust web frontend.
#![allow(clippy::enum_variant_names)]

use lazy_static::lazy_static;
use yew::prelude::*;
use yew_router::prelude::*;

use pages::{
    about::About, blog::Blog, not_found::NotFound, post_view::PostView, root::Root,
    violence::Violence,
};
use router::Route;

mod errors;
mod models;
mod pages;
mod router;
mod traits;
mod utils;

lazy_static! {
    /// The path to the GIF where a character is typing nonsense into a computer.
    static ref FAVICON_GIF: &'static str = "../../nonsense.gif";
}

/// The main application entry point.
#[function_component(Main)]
fn app() -> Html {
    let version_number = env!("CARGO_PKG_VERSION");

    html! {
        <div class="crt" style="display: flex; flex-direction: column; min-height: 100vh;">
          <BrowserRouter>
            <Switch<Route> render={switch} />
          </BrowserRouter>
          <footer>
            <small class="footer-small">
              { format!("v{version_number} | est. 2023 |") }
              <span class="github-svg-container">
                <a href="https://github.com/JosephLai241/stacc">
                  <img src="../../github.svg" style="margin-bottom: 4px;" alt="GitHub link"/>
                </a>
              </span>
            </small>
          </footer>
        </div>
    }
}

/// Contains the router switch -- maps routes to the functional components for each page.
fn switch(route: Route) -> Html {
    match route {
        Route::About => html! { <About /> },
        Route::Blog => html! { <Blog /> },
        Route::NotFound => html! { <NotFound /> },
        Route::PostView { post_id } => html! { <PostView post_id={post_id} /> },
        Route::Root => html! { <Root /> },
        Route::Violence => html! { <Violence /> },
    }
}

/// Run the frontend via `yew`.
fn main() {
    yew::Renderer::<Main>::new().render();
}
