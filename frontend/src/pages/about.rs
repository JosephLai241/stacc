//! The about page of this site.

use gloo_console::error;
use yew::prelude::*;

use crate::pages::utils;
use crate::utils::open_graph::{OpenGraphTag, PageType};
use crate::utils::{background, open_graph};
use crate::FAVICON_GIF;

/// The about component.
#[function_component(About)]
pub fn about() -> Html {
    background::set_background(false);
    gloo_utils::document().set_title("jl | about");

    open_graph::set_open_graph_tag(OpenGraphTag::Description("about me".to_string()))
        .unwrap_or_else(|error| error!(error.to_string()));
    open_graph::set_open_graph_tag(OpenGraphTag::ImageLink(FAVICON_GIF.to_string()))
        .unwrap_or_else(|error| error!(error.to_string()));
    open_graph::set_open_graph_tag(OpenGraphTag::PageType(PageType::Website))
        .unwrap_or_else(|error| error!(error.to_string()));
    open_graph::set_open_graph_tag(OpenGraphTag::Title("jl | about".to_string()))
        .unwrap_or_else(|error| error!(error.to_string()));
    open_graph::set_open_graph_tag(OpenGraphTag::Url("https://josephlai.dev/about".to_string()))
        .unwrap_or_else(|error| error!(error.to_string()));

    let intro = r#"Hello! My name is Joseph Lai and I am a software engineer based
in Chicago. Welcome to my portfolio site/developer blog."#;
    let second_paragraph = r#"This site serves as both a form of artistic expression
and a place to blog about all things computer science."#;

    let page_body = html! {
        <div class="container fade-in-slide-down">
          <div class="left-half-container">
            <p>{ intro.to_string() }</p>
            <p>{ second_paragraph.to_string() }</p>
            <p>{ "This site was written entirely in " }
              <a href="https://www.rust-lang.org/">{ "Rust" }</a>
              { " by using " }
              <a href="https://yew.rs"><code>{ "Yew" }</code></a>
              { " to implement the frontend and " }
              <a href="https://actix.rs"><code>{ "Actix Web" }</code></a>
              { " to implement the API." }
            </p>
            <div class="image-container">
              <a class="social-icon" href="https://github.com/JosephLai241">
                <img src="../../github.png" alt="GitHub" />
              </a>
              <a class="social-icon" href="https://www.linkedin.com/in/joseph-lai-86390a137/">
                <img src="../../linkedin.png" alt="LinkedIn" />
              </a>
            </div>
          </div>
        </div>
    };

    html! {
        {
            utils::create_page_with_nav(None, page_body)
        }
    }
}
