//! `stacc` -- A Rust web frontend.

use yew::prelude::*;
use yew_router::prelude::*;

use pages::root::Root;
use router::Route;

mod errors;
mod pages;
mod router;
mod utils;

/// The main application entry point.
#[function_component(Main)]
fn app() -> Html {
    let version_number = env!("CARGO_PKG_VERSION");

    html! {
        <div style="display: flex; flex-direction: column; min-height: 100vh;">
          <BrowserRouter>
            <Switch<Route> render={switch} />
          </BrowserRouter>
          <footer>
            <small class="footer-small"
              >{ format!("v{version_number} | est. 2023") }</small
            >
          </footer>
        </div>
    }
}

/// Contains the router switch -- maps routes to the functional components for each page.
fn switch(route: Route) -> Html {
    match route {
        Route::About => {
            unimplemented!()
        }
        Route::Blog => {
            unimplemented!()
        }
        Route::NotFound => {
            unimplemented!()
        }
        Route::PostView { post_id } => {
            unimplemented!()
        }
        Route::Root => html! {
            <Root />
        },
    }
}

/// Run the frontend via `yew`.
fn main() {
    yew::Renderer::<Main>::new().render();
}
