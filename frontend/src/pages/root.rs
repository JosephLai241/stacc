//! The root page (landing page) of this site.

use chrono::Local;
use futures_util::{future::ready, StreamExt};
use gloo_timers::future::IntervalStream;
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
                  { "JL" }
                </a>
              </h1>
            </i>
          </b>
          <Clock />
          <h3><a href="/blog">{"blog"}</a></h3>
          <h3><a href="/about">{"about"}</a></h3>
        </div>
    }
}

/// A clock component that renders Chicago time. This timestamp updates every second.
#[function_component(Clock)]
fn clock() -> Html {
    let timestamp = Local::now().format("%Y/%m/%d %H:%M:%S CHICAGO").to_string();

    wasm_bindgen_futures::spawn_local(async move {
        IntervalStream::new(1_000)
            .for_each(|_| {
                if let Some(clock_element) = gloo_utils::document().get_element_by_id("clock") {
                    let new_timestamp =
                        Local::now().format("%Y/%m/%d %H:%M:%S CHICAGO").to_string();

                    clock_element.set_inner_html(&new_timestamp);
                }
                ready(())
            })
            .await
    });

    html! {
        <p class="clock" id="clock">{ timestamp }</p>
    }
}
