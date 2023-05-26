//! Contains models for posts.

use serde::{Deserialize, Serialize};

/// Contains all post data passed to/from the API and frontend.
#[derive(Debug, Deserialize, Serialize)]
pub struct PostData {
    /// The body of the post.
    pub body: String,
    /// The created timestamp.
    pub created: String,
    /// The edited timestamp.
    pub edited: Option<String>,
    /// The post's ID.
    pub post_id: String,
    /// The preview image link.
    pub preview_image_link: String,
    /// The preview summary.
    pub preview_summary: String,
    /// The title of the post.
    pub title: String,
    /// The number of views.
    pub view_count: i32,
}

/// This struct holds a `Vec<PostData>` containing all posts.
#[derive(Debug, Deserialize, Serialize)]
pub struct AllPosts {
    /// Contains all posts from newest to oldest.
    pub posts: Vec<PostData>,
}
