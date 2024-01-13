//! The page containing the blog post.

use gloo_console::error;
use gloo_net::http::Request;
use yew::prelude::*;

use crate::{
    models::{blog::PostData, response::Response},
    pages::utils::{self, Loading},
    utils::{
        background,
        open_graph::{self, OpenGraphTag, PageType},
    },
    FAVICON_GIF,
};

/// Contains properties that may be passed into the `PostView` component.
#[derive(Debug, Eq, PartialEq, Properties)]
pub struct PostViewProps {
    /// The post's ID. This is used to query the API for that particular post's data.
    pub post_id: String,
}

/// The post view page.
#[function_component(PostView)]
pub fn post_view(props: &PostViewProps) -> Html {
    background::set_background(true);

    let post_id = props.post_id.clone();

    let is_loading = use_state(|| true);
    let get_post_response = use_state(|| None);
    {
        let is_loading = is_loading.clone();
        let get_post_response = get_post_response.clone();

        use_effect_with_deps(
            move |_| {
                open_graph::set_open_graph_tag(OpenGraphTag::PageType(PageType::Article))
                    .unwrap_or_else(|error| error!(error.to_string()));
                open_graph::set_open_graph_tag(OpenGraphTag::Url(format!(
                    "https://josephlai.dev/blog/{post_id}"
                )))
                .unwrap_or_else(|error| error!(error.to_string()));

                wasm_bindgen_futures::spawn_local(async move {
                    match Request::get(&format!("/api/blog/post/{post_id}"))
                        .send()
                        .await
                    {
                        Ok(response) => match response.status() {
                            200 => response.json::<PostData>().await.map_or_else(
                                |error| {
                                    is_loading.set(false);

                                    open_graph::set_open_graph_tag(OpenGraphTag::Description(
                                        "something fucked up".to_string(),
                                    ))
                                    .unwrap_or_else(|error| error!(error.to_string()));
                                    open_graph::set_open_graph_tag(OpenGraphTag::ImageLink(
                                        FAVICON_GIF.to_string(),
                                    ))
                                    .unwrap_or_else(|error| error!(error.to_string()));
                                    open_graph::set_open_graph_tag(OpenGraphTag::Title(
                                        "jl | blog | fuck".to_string(),
                                    ))
                                    .unwrap_or_else(|error| error!(error.to_string()));

                                    get_post_response.set(Some(Err(
                                        Response::status_500_with_message(format!(
                                            "UNABLE TO PARSE THE POST DATA TO JSON: {error}"
                                        )),
                                    )));
                                },
                                |post_data| {
                                    is_loading.set(false);

                                    open_graph::set_open_graph_tag(OpenGraphTag::Description(
                                        post_data.preview_summary.clone(),
                                    ))
                                    .unwrap_or_else(|error| error!(error.to_string()));
                                    open_graph::set_open_graph_tag(OpenGraphTag::ImageLink(
                                        post_data.preview_image_link.clone(),
                                    ))
                                    .unwrap_or_else(|error| error!(error.to_string()));
                                    open_graph::set_open_graph_tag(OpenGraphTag::Title(format!(
                                        "jl | blog | {}",
                                        post_data.title
                                    )))
                                    .unwrap_or_else(|error| error!(error.to_string()));

                                    get_post_response.set(Some(Ok(post_data)));
                                },
                            ),
                            _ => response.json::<Response>().await.map_or_else(
                                |_| {
                                    is_loading.set(false);

                                    open_graph::set_open_graph_tag(OpenGraphTag::Description(
                                        "something fucked up".to_string(),
                                    ))
                                    .unwrap_or_else(|error| error!(error.to_string()));
                                    open_graph::set_open_graph_tag(OpenGraphTag::ImageLink(
                                        FAVICON_GIF.to_string(),
                                    ))
                                    .unwrap_or_else(|error| error!(error.to_string()));
                                    open_graph::set_open_graph_tag(OpenGraphTag::Title(
                                        "jl | blog | fuck".to_string(),
                                    ))
                                    .unwrap_or_else(|error| error!(error.to_string()));

                                    get_post_response.set(Some(Err(
                                        Response::status_500_with_message(
                                            "No API response.".to_string(),
                                        ),
                                    )));
                                },
                                |response| {
                                    is_loading.set(false);

                                    open_graph::set_open_graph_tag(OpenGraphTag::Description(
                                        "something fucked up".to_string(),
                                    ))
                                    .unwrap_or_else(|error| error!(error.to_string()));
                                    open_graph::set_open_graph_tag(OpenGraphTag::ImageLink(
                                        FAVICON_GIF.to_string(),
                                    ))
                                    .unwrap_or_else(|error| error!(error.to_string()));
                                    open_graph::set_open_graph_tag(OpenGraphTag::Title(
                                        "jl | blog | fuck".to_string(),
                                    ))
                                    .unwrap_or_else(|error| error!(error.to_string()));

                                    get_post_response.set(Some(Err(response)));
                                },
                            ),
                        },
                        Err(error) => {
                            is_loading.set(false);

                            get_post_response.set(Some(Err(Response::status_500_with_message(
                                format!("UNABLE TO GET THE POST FROM THE API: {error}"),
                            ))));
                        }
                    }
                });

                || ()
            },
            (),
        );
    }

    let response = get_post_response
        .as_ref()
        .unwrap_or(&Ok(PostData::from_post_id(props.post_id.clone())))
        .to_owned();

    let post_body = match response {
        Ok(post_data) => {
            let post_skeleton = html! {
                <div class="fade-in-slide-down">
                  <div class="d-flex flex-column">
                    <div style="margin-bottom: 1rem !important;">
                      <img alt={"FUCK. COULDN'T LOAD THE IMAGE."} src={ post_data.preview_image_link.clone() } />
                    </div>
                    <h1 class="error-text" style="margin-bottom: 1rem !important;">
                      { post_data.title.clone() }
                    </h1>
                    <h5>
                      { post_data.created }
                    </h5>
                    {
                      if let Some(edited) = post_data.edited {
                          html! {
                              <div>
                                <small>
                                  { format!("[ edited {edited} ]") }
                                </small>
                              </div>
                          }
                      } else {
                          html! { <></> }
                      }
                    }
                    <div style="display: flex; margin-bottom: 1rem !important;">
                      <span class="badge p-2 blog-post-topic-badge">
                        { post_data.topic }
                      </span>
                    </div>
                    <p class="blog-post-preview-summary">
                      { post_data.preview_summary }
                    </p>
                    <div class="rusty-line-thicc"></div>
                    <div id="post-content"></div>
                  </div>
                </div>
            };

            gloo_utils::document().set_title(&format!("jl | blog | {}", post_data.title));

            utils::inject_post_body("post-content", &post_data.body);

            post_skeleton
        }
        Err(error) => {
            gloo_utils::document().set_title("jl | blog | fuck");

            html! {
                <div class="fade-in-slide-down">
                  <h1 class="error-text"><i>{"fuck"}</i></h1>
                  <h4 style="color: #832700;">
                    <b>{ format!("HTTP {}", error.status_code) }</b>
                  </h4>
                  <small>{ error.message }</small>
                </div>
            }
        }
    };

    utils::create_page_with_nav(
        Some("/blog".to_string()),
        html! {
            <div class="blog-post fade-in-slide-down">
            {
                if *is_loading {
                    html! { <Loading /> }
                } else {
                    post_body
                }
            }
            </div>
        },
    )
}
