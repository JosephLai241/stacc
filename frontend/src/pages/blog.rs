//! The blog page containing post cards.

use gloo_console::error;
use gloo_net::http::Request;
use yew::prelude::*;

use crate::models::{
    blog::{AllPosts, PostData},
    response::Response,
};
use crate::pages::utils::{self, Loading};
use crate::utils::{
    background,
    open_graph::{self, OpenGraphTag, PageType},
};
use crate::FAVICON_GIF;

/// The blog page.
#[function_component(Blog)]
pub fn blog() -> Html {
    background::set_background(false);
    gloo_utils::document().set_title("jl | blog");

    let is_loading = use_state(|| true);
    let get_posts_response = use_state(|| None);
    {
        let is_loading = is_loading.clone();
        let get_posts_response = get_posts_response.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    open_graph::set_open_graph_tag(OpenGraphTag::Description(
                        "my blog".to_string(),
                    ))
                    .unwrap_or_else(|error| error!(error.to_string()));
                    open_graph::set_open_graph_tag(OpenGraphTag::ImageLink(
                        FAVICON_GIF.to_string(),
                    ))
                    .unwrap_or_else(|error| error!(error.to_string()));
                    open_graph::set_open_graph_tag(OpenGraphTag::PageType(PageType::Website))
                        .unwrap_or_else(|error| error!(error.to_string()));
                    open_graph::set_open_graph_tag(OpenGraphTag::Title("jl | blog".to_string()))
                        .unwrap_or_else(|error| error!(error.to_string()));
                    open_graph::set_open_graph_tag(OpenGraphTag::Url(
                        "https://josephlai.dev/blog".to_string(),
                    ))
                    .unwrap_or_else(|error| error!(error.to_string()));

                    match Request::get("/api/blog/posts").send().await {
                        Ok(response) => response.json::<AllPosts>().await.map_or_else(
                            |error| {
                                is_loading.set(false);
                                get_posts_response.set(Some(Err(
                                    Response::status_500_with_message(format!(
                                        "UNABLE TO PARSE THE POSTS TO JSON: {error}"
                                    )),
                                )))
                            },
                            |mut all_posts| {
                                all_posts.posts = all_posts.posts.into_iter().rev().collect();

                                is_loading.set(false);
                                get_posts_response.set(Some(Ok(all_posts)))
                            },
                        ),
                        Err(error) => {
                            is_loading.set(false);
                            get_posts_response.set(Some(Err(Response::status_500_with_message(
                                format!("UNABLE TO GET POSTS FROM THE API: {error}"),
                            ))))
                        }
                    }
                });

                || ()
            },
            (),
        )
    }

    let response = get_posts_response
        .as_ref()
        .unwrap_or(&Ok(AllPosts { posts: vec![] }))
        .to_owned();

    let blog_body = match response {
        Ok(all_posts) => {
            if !all_posts.posts.is_empty() {
                html! {
                    <div class="container fade-in-slide-down">
                      <div class="card-columns">
                        { for all_posts.posts.iter().map(make_bootstrap_card) }
                      </div>
                    </div>
                }
            } else {
                html! {
                    <div class="container fade-in-slide-down">
                      <p class="error-text">{ "nature does not hurry, yet everything is accomplished" }</p>
                    </div>
                }
            }
        }
        Err(error) => html! {
            <div class="container fade-in-slide-down">
              <div class="left-half-container">
                <div class="row">
                  <h1 class="error-text"><i>{ "fuck" }</i></h1>
                </div>
                <div class="row">
                  <h4 style="color: #832700;">
                    <b>{ format!("HTTP {}", error.status_code) }</b>
                  </h4>
                </div>
                <div class="row">
                  <p>
                    { error.message.clone() }
                  </p>
                </div>
              </div>
            </div>
        },
    };

    utils::create_page_with_nav(
        None,
        if *is_loading {
            html! { <Loading /> }
        } else {
            blog_body
        },
    )
}

/// Create a Bootstrap Card for each `PostData` struct.
fn make_bootstrap_card(post_data: &PostData) -> Html {
    html! {
        <div class="col">
            <a
              href={ format!("/blog/post/{}", post_data.post_id.clone()) }
              style="text-decoration: none;"
            >
              <div class="card rusty-card">
                <img
                  alt="FUCK. COULDN'T LOAD THE IMAGE."
                  class="card-img-top"
                  src={ post_data.preview_image_link.clone() }/>
                <div class="card-body">
                  <h5 class="card-title">
                    { post_data.title.clone() }
                  </h5>
                  <div style="display: flex; margin-bottom: 1rem !important;">
                    <span class="badge p-2 blog-post-topic-badge">
                      { post_data.topic.clone() }
                    </span>
                  </div>
                  <div class="rusty-card-line"></div>
                    <h6 class="card-text">
                      { post_data.preview_summary.clone() }
                    </h6>
                  </div>
                  <div class="card-footer rusty-card-footer">
                    <small>{ post_data.created.clone() }</small>
                  </div>
                </div>
            </a>
        </div>
    }
}
