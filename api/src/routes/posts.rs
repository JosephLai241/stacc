//! Contains all routes pertaining to posts.

use actix_web::{
    get,
    web::{Data, Path},
    HttpRequest, HttpResponse,
};
use futures_util::stream::StreamExt;
use log::error;
use mongodb::bson::doc;

use crate::{
    errors::StaccResponseError,
    middleware,
    models::post::{AllPosts, PostData},
    utils::mongo::Mongo,
};

/// Get all posts from MongoDB.
#[get("/posts")]
pub async fn get_all_posts(
    mongo: Data<Mongo>,
    request: HttpRequest,
) -> Result<HttpResponse, StaccResponseError> {
    if let Err(error) = middleware::log_visitor_data(&mongo, &request).await {
        error!("{}", error);
    }

    let mut posts: Vec<PostData> = Vec::new();

    match mongo.posts_collection.find(doc! {}, None).await {
        Ok(mut cursor) => {
            while let Some(document) = cursor.next().await {
                match document {
                    Ok(post) => {
                        posts.push(post);
                    }
                    Err(error) => {
                        return Err(StaccResponseError::MongoDBError {
                            error: error.to_string(),
                        })
                    }
                }
            }
        }
        Err(error) => {
            return Err(StaccResponseError::MongoDBError {
                error: error.to_string(),
            })
        }
    }

    Ok(HttpResponse::Ok().json(AllPosts { posts }))
}

/// Get a single post from MongoDB.
#[get("/post/{post_id}")]
pub async fn get_single_post(
    mongo: Data<Mongo>,
    post_id: Path<String>,
    request: HttpRequest,
) -> Result<HttpResponse, StaccResponseError> {
    if let Err(error) = middleware::log_visitor_data(&mongo, &request).await {
        error!("{}", error);
    }

    let post_id = post_id.into_inner();

    let find_result = mongo
        .posts_collection
        .find_one_and_update(
            doc! { "post_id": post_id.clone() },
            doc! { "$inc": { "view_count": 1 } },
            None,
        )
        .await;

    match find_result {
        Ok(Some(post)) => {
            if let Err(error) = middleware::log_post_view(&mongo, &post_id, &request).await {
                error!("{}", error);
            }

            Ok(HttpResponse::Ok().json(post))
        }
        Ok(None) => Err(StaccResponseError::MongoDBSearchError {
            error: "Post not found!".to_string(),
        }),
        Err(error) => Err(StaccResponseError::MongoDBError {
            error: error.to_string(),
        }),
    }
}
