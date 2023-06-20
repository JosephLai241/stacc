//! Contains miscellaneous routes for the API.

use std::time::Duration;

use actix_web::{
    cookie::{Cookie, SameSite},
    get,
    web::Data,
    HttpRequest, HttpResponse,
};
use lazy_static::lazy_static;
use log::error;
use mongodb::{bson::doc, options::FindOneOptions};
use rand::Rng;
use tokio::time;

use crate::{
    errors::StaccResponseError,
    middleware,
    models::data::{BackgroundGIF, Story},
    utils::{environment::EnvironmentVariables, mongo::Mongo},
};

lazy_static! {
    /// The default fallback GIF if selecting a random GIF from MongoDB fails.
    /// This GIF is Takumi doing a heel-toe downshift in one of the best cars ever made -- the
    /// Subaru WRX 🥴.
    static ref FALLBACK_GIF: &'static str = "https://imgur.com/FgJDNsx.gif";
    /// The default fallback story if selecting a random story from MongoDB fails.
    static ref FALLBACK_STORY: &'static str = "If you don’t like the road you’re walking, pave another one. Except for this one.";
}

/// Create a cookie that stores the background GIF link. Returns `Some(Cookie)` if able to retrieve
/// the `EnvironmentVariables::StaccDomain` value, otherwise returns `None`.
fn create_background_cookie<'cookie>(background_gif: String) -> Option<Cookie<'cookie>> {
    EnvironmentVariables::StaccDomain
        .env_var()
        .ok()
        .map_or_else(
            || None,
            |domain| {
                Some(
                    Cookie::build("background", background_gif)
                        .domain(domain)
                        .expires(None)
                        .path("/")
                        .same_site(SameSite::Strict)
                        .finish(),
                )
            },
        )
}

/// Get the background GIF by choosing a random link stored in the backgrounds collection.
#[get("/background")]
pub async fn get_background_gif(
    mongo: Data<Mongo>,
    request: HttpRequest,
) -> Result<HttpResponse, StaccResponseError> {
    if let Err(error) = middleware::log_visitor_data(&mongo, &request).await {
        error!("{}", error);
    }

    // Delay execution for one second to allow the static GIF to render on the
    // frontend. Shit's just too fast man.
    time::sleep(Duration::from_secs(1)).await;

    let document_count = mongo
        .backgrounds_collection
        .count_documents(doc! {}, None)
        .await
        .map_err(|error| StaccResponseError::MongoDBError {
            error: error.to_string(),
        })?;

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..document_count);

    let background_gif = mongo
        .backgrounds_collection
        .find_one(
            doc! {},
            FindOneOptions::builder().skip(random_index).build(),
        )
        .await
        .ok()
        .flatten()
        .map_or_else(
            || BackgroundGIF {
                link: FALLBACK_GIF.to_string(),
            },
            |gif| gif,
        );

    match create_background_cookie(background_gif.link.clone()) {
        Some(cookie) => Ok(HttpResponse::Ok().cookie(cookie).json(background_gif)),
        None => Ok(HttpResponse::Ok().json(background_gif)),
    }
}

/// Get a 404 page story by choosing a random story stored in the stories collection.
#[get("/story")]
pub async fn story(
    mongo: Data<Mongo>,
    request: HttpRequest,
) -> Result<HttpResponse, StaccResponseError> {
    if let Err(error) = middleware::log_visitor_data(&mongo, &request).await {
        error!("{}", error);
    }

    let document_count = mongo
        .stories_collection
        .count_documents(doc! {}, None)
        .await
        .map_err(|error| StaccResponseError::MongoDBError {
            error: error.to_string(),
        })?;

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..document_count);

    let story = mongo
        .stories_collection
        .find_one(
            doc! {},
            FindOneOptions::builder().skip(random_index).build(),
        )
        .await
        .ok()
        .flatten()
        .map_or_else(
            || Story {
                story: FALLBACK_STORY.to_string(),
            },
            |story| story,
        );

    Ok(HttpResponse::Ok().json(story))
}
