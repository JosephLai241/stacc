//! Contains all routes pertaining to posts.

use actix_web::{get, post, HttpRequest, HttpResponse};

use crate::{errors::StaccResponseError, models::post::PostData};

/// Get all posts from MongoDB.
#[get("/posts")]
pub async fn get_all_posts(request: HttpRequest) -> Result<HttpResponse, StaccResponseError> {
    let mut post_list: Vec<PostData> = Vec::new();
}
