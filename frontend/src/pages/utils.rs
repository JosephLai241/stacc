//! Contains miscellaneous utilities for creating pages.

use gloo_console::error;
use js_sys::Function;
use pulldown_cmark::{html, Options, Parser};
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlTableCellElement, HtmlTableElement, HtmlTableRowElement, MutationObserver,
    MutationObserverInit,
};
use yew::prelude::*;

use crate::errors::StaccError;

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

/// Create a table from the data in a given `Vec<(String, i32)>`.
pub fn create_table_from_data(
    table_header: (&str, &str),
    data: &Vec<(String, i32)>,
) -> Result<HtmlTableElement, StaccError> {
    let document = gloo_utils::document();

    let table = document
        .create_element("table")?
        .dyn_into::<HtmlTableElement>()?;
    table.set_class_name("data-table");

    let header_row = document
        .create_element("tr")?
        .dyn_into::<HtmlTableRowElement>()?;
    header_row.set_class_name("data-table-header-row");

    let key_header = document
        .create_element("th")?
        .dyn_into::<HtmlTableCellElement>()?;
    key_header.set_inner_text(&table_header.0);
    key_header.set_class_name("data-table-header-cell");

    let value_header = document
        .create_element("th")?
        .dyn_into::<HtmlTableCellElement>()?;
    value_header.set_inner_text(&table_header.1);
    value_header.set_class_name("data-table-header-cell");

    header_row.append_child(&key_header);
    header_row.append_child(&value_header);

    table.append_child(&header_row);

    for key_value in data.into_iter() {
        let row = document
            .create_element("tr")?
            .dyn_into::<HtmlTableRowElement>()?;
        row.set_class_name("data-table-row");

        let key = document
            .create_element("td")?
            .dyn_into::<HtmlTableCellElement>()?;
        key.set_inner_text(&key_value.0);
        key.set_class_name("data-table-left-cell");

        let value = document
            .create_element("td")?
            .dyn_into::<HtmlTableCellElement>()?;
        value.set_inner_text(&key_value.1.to_string());
        value.set_class_name("data-table-right-cell-left-border");

        row.append_child(&key);
        row.append_child(&value);

        table.append_child(&row);
    }

    Ok(table)
}
