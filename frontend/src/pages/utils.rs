//! Contains miscellaneous utilities for creating pages.

use gloo_console::error;
use js_sys::Function;
use pulldown_cmark::{html, Options, Parser};
use web_sys::{MutationObserver, MutationObserverInit};
use yew::prelude::*;

/// Convert the post's body from Markdown to HTML.
fn create_post_body(post_body: &str) -> String {
    let mut parser_options = Options::empty();
    parser_options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    parser_options.insert(Options::ENABLE_SMART_PUNCTUATION);
    parser_options.insert(Options::ENABLE_STRIKETHROUGH);
    parser_options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(post_body, parser_options);

    let mut post_body = String::new();
    html::push_html(&mut post_body, parser);

    post_body
}

/// Parse the post's body Markdown into HTML, then inject the HTML into the page.
pub fn inject_post_body(element_id: &str, post_body: &str) {
    let post_body = create_post_body(post_body);

    let arguments = "mutations, observer";
    let body = &format!(
        r#"
            const contentDiv = document.getElementById('{element_id}');
            if (contentDiv) {{
              contentDiv.innerHTML = `{}`;
              hljs.highlightAll();

              observer.disconnect();

              return;
            }}
        "#,
        &post_body
    );

    MutationObserver::new(&Function::new_with_args(arguments, body)).map_or_else(
        |error| {
            error!("An error occurred while instantiating a MutationObserver!");
            error!(format!("{:#?}", error));
        },
        |observer| {
            if let Err(error) = observer.observe_with_options(
                &gloo_utils::body().get_root_node(),
                MutationObserverInit::new().child_list(true).subtree(true),
            ) {
                error!("Observation failure!");
                error!(format!("{:#?}", error));
            }
        },
    );
}

/// Create `Html` containing the navigation bar and an `Html` component underneath it.
pub fn create_page_with_nav(back_button_href: Option<String>, page_body: Html) -> Html {
    html! {
        <div>
          <div class="container p-2">
            <div class="container d-flex fauxbar mb-5 mt-3">
            {
                match back_button_href {
                    Some(href) => html! {
                        <div>
                          <b>
                            <i>
                              <h1>
                                <a
                                  class="title-text"
                                  href={ href }
                                  style="text-decoration: none; color: #cfcfcf"
                                >
                                  { "☚" }
                                </a>
                              </h1>
                            </i>
                          </b>
                        </div>
                    },
                    None => html! { <></> }
                }
            }
              <div style="margin-left: 5%; margin-right: auto">
                <b>
                  <i>
                    <h1>
                      <a
                        class="title-text"
                        href="/"
                        style="text-decoration: none; color: #cfcfcf"
                      >
                        { "JL" }
                      </a>
                    </h1>
                  </i>
                </b>
              </div>
            </div>
            { page_body }
          </div>
        </div>
    }
}

/// The loading animation.
#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div style="display: flex; justify-content: center;">
          <h2 class="animated-loading">
            <i>{"☝"}</i>
          </h2>
        </div>
    }
}
