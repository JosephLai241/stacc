//! The root page (landing page) of this site.

use yew::prelude::*;

use crate::utils::background;

/// The root component.
#[function_component(Root)]
pub fn root() -> Html {
    background::set_background(true);
    gloo_utils::document().set_title("josephlai");

    html! {
        <div class="root-container fade-in-slide-down">
          <b>
            <i>
              <h1>
                <a
                  class="root-title title-text"
                  href="/"
                  style="text-decoration: none; color: #cfcfcf"
                >
                  { "JOSEPH LAI" }
                </a>
              </h1>
            </i>
          </b>
          <h3><a href="/blog">{"blog"}</a></h3>
          <h3><a href="/about">{"about"}</a></h3>
        </div>
    }
}
