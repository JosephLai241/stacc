//! The 404 not found page.

use gloo_console::error;
use gloo_net::http::Request;
use yew::prelude::*;

use crate::{
    models::{response::Response, story::Story},
    pages::utils::{self, Loading},
    utils::{
        background,
        open_graph::{self, OpenGraphTag, PageType},
    },
    FAVICON_GIF,
};

/// The 404 Not Found page.
#[function_component(NotFound)]
pub fn not_found() -> Html {
    background::set_background(false);
    gloo_utils::document().set_title("jl | 404");

    let is_loading = use_state(|| true);
    let get_story_response = use_state(|| None);
    {
        let is_loading = is_loading.clone();
        let get_story_response = get_story_response.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    open_graph::set_open_graph_tag(OpenGraphTag::Description(
                        "go the fuck home".to_string(),
                    ))
                    .unwrap_or_else(|error| error!(error.to_string()));
                    open_graph::set_open_graph_tag(OpenGraphTag::ImageLink(
                        FAVICON_GIF.to_string(),
                    ))
                    .unwrap_or_else(|error| error!(error.to_string()));
                    open_graph::set_open_graph_tag(OpenGraphTag::PageType(PageType::Website))
                        .unwrap_or_else(|error| error!(error.to_string()));
                    open_graph::set_open_graph_tag(OpenGraphTag::Title("jl | 404".to_string()))
                        .unwrap_or_else(|error| error!(error.to_string()));

                    match Request::get("/api/story").send().await {
                        Ok(response) => match response.status() {
                            200 => match response.json::<Story>().await {
                                Ok(story) => {
                                    is_loading.set(false);
                                    get_story_response.set(Some(Ok(story)));
                                }
                                Err(error) => {
                                    is_loading.set(false);
                                    get_story_response.set(Some(Err(Response {
                                        message: format!(
                                            "UNABLE TO PARSE STORY TO JSON: {}",
                                            error.to_string()
                                        ),
                                        status_code: 500,
                                    })))
                                }
                            },
                            _ => match response.json::<Response>().await {
                                Ok(response) => {
                                    is_loading.set(false);
                                    get_story_response.set(Some(Err(response)))
                                }
                                Err(error) => {
                                    is_loading.set(false);
                                    get_story_response.set(Some(Err(Response {
                                        message: format!(
                                            "UNABLE TO PARSE THE API RESPONSE TO JSON: {error}"
                                        ),
                                        status_code: 500,
                                    })))
                                }
                            },
                        },
                        Err(error) => {
                            is_loading.set(false);
                            get_story_response.set(Some(Err(Response {
                                message: format!(
                                    "UNABLE TO GET A STORY FROM THE API: {}",
                                    error.to_string()
                                ),
                                status_code: 500,
                            })));
                        }
                    }
                });

                || ()
            },
            (),
        )
    }

    let response = get_story_response
        .as_ref()
        .unwrap_or(&Ok(Story::default()))
        .to_owned();

    let story = response.map_or(
        html! {
            <div class="container fade-in-slide-down">
              <p>
                { Story::default().story }
              </p>
              <p>{ "In other words, what the fuck are you doing? You have reached the 404 page. Go home." }</p>
            </div>
        },
        |story| {
            let body = html! {
                <div class="container fade-in-slide-down">
                  <div id="404-story"></div>
                  <p>{ "In other words, what the fuck are you doing? You have reached the 404 page. Go home." }</p>
                </div>
            };

            utils::inject_post_body("404-story", &story.story);

            body
        },
    );

    utils::create_page_with_nav(
        None,
        html! {
            <div class="left-half-container">
            {
                if *is_loading {
                    html! {
                        <div class="d-flex flex-column centered-loading">
                          <Loading />
                        </div>
                    }
                } else {
                    html! { story }
                }
            }
            </div>
        },
    )
}
