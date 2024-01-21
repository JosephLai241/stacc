//! The root page (landing page) of this site.

use chrono::{TimeZone, Utc};
use chrono_tz::America::Chicago;
use futures_util::{future::ready, StreamExt};
use gloo_console::error;
use gloo_timers::future::IntervalStream;
use lazy_static::lazy_static;
use yew::prelude::*;

use crate::{
    utils::{
        background,
        open_graph::{self, OpenGraphTag, PageType},
    },
    FAVICON_GIF,
};

lazy_static! {
    /// The timestamp format to display in the `Clock` component.
    static ref TIMESTAMP_FORMAT: &'static str = "%Y/%m/%d %H:%M:%S CHICAGO";
}

/// The root component.
#[function_component(Root)]
pub fn root() -> Html {
    background::set_background(true);
    gloo_utils::document().set_title("jl");

    open_graph::set_open_graph_tag(OpenGraphTag::Description(
        "a form of artistic expression and a place to blog about CS".to_string(),
    ))
    .unwrap_or_else(|error| error!(error.to_string()));
    open_graph::set_open_graph_tag(OpenGraphTag::ImageLink(FAVICON_GIF.to_string()))
        .unwrap_or_else(|error| error!(error.to_string()));
    open_graph::set_open_graph_tag(OpenGraphTag::PageType(PageType::Website))
        .unwrap_or_else(|error| error!(error.to_string()));
    open_graph::set_open_graph_tag(OpenGraphTag::Title("jl".to_string()))
        .unwrap_or_else(|error| error!(error.to_string()));
    open_graph::set_open_graph_tag(OpenGraphTag::Url("https://josephlai.dev".to_string()))
        .unwrap_or_else(|error| error!(error.to_string()));

    html! {
        <div class="root-container fade-in-slide-down">
          <b>
            <i>
              <h1>
                <a
                  class="root-title title-text"
                  href="/"
                  style="text-decoration: none; color: #b0b0b0;"
                >
                  { "JL" }
                </a>
              </h1>
            </i>
          </b>
          <Clock />
          <ShadesOfRust />
          <div style="margin-bottom: 50px;"></div>
          <h3><a href="/blog">{"blog"}</a></h3>
          <h3><a href="/violence">{"violence"}</a></h3>
          <h3><a href="/about">{"about"}</a></h3>
        </div>
    }
}

/// A clock component that renders Chicago time. This timestamp updates every second.
#[function_component(Clock)]
fn clock() -> Html {
    let current_utc = Utc::now().naive_utc().timestamp_nanos();
    let timestamp = Chicago
        .timestamp_nanos(current_utc)
        .format(&TIMESTAMP_FORMAT)
        .to_string();

    wasm_bindgen_futures::spawn_local(async move {
        IntervalStream::new(1_000)
            .for_each(|_| {
                if let Some(clock_element) = gloo_utils::document().get_element_by_id("clock") {
                    let current_utc = Utc::now().naive_utc().timestamp_nanos();
                    let new_timestamp = Chicago
                        .timestamp_nanos(current_utc)
                        .format(&TIMESTAMP_FORMAT)
                        .to_string();

                    clock_element.set_inner_html(&new_timestamp);
                }

                ready(())
            })
            .await
    });

    html! {
        <p class="clock pulse" id="clock">{ timestamp }</p>
    }
}

/// A simple color palette containing various shades of rust.
#[function_component(ShadesOfRust)]
fn shades_of_rust() -> Html {
    html! {
        <>
          <a href="https://www.rust-lang.org/">
            <div class="color-box" style="background-color: #b7410e;"></div>
          </a>
          <a href="https://www.google.com/search?q=%23B7410E">
            <div class="color-box" style="background-color: #a53b0d;"></div>
          </a>
          <a href="https://i.imgur.com/thdjm7y.jpg">
            <div class="color-box" style="background-color: #92340b;"></div>
          </a>
          <a href="https://i.imgur.com/mzCEmev.jpg">
            <div class="color-box" style="background-color: #802e0a;"></div>
          </a>
          <a href="https://www.teepublic.com/t-shirt/27609036-i-hate-java">
            <div class="color-box" style="background-color: #6e2708;"></div>
          </a>
          <a href="https://videos.danksquad.org/w/cca8b880-87d2-4ce5-815f-7e2c020d5b75">
            <div class="color-box" style="background-color: #5c2107;"></div>
          </a>
        </>
    }
}
