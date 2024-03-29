//! Contains all models pertaining to blog posts.

use serde::Deserialize;

/// This struct holds all posts in a `Vec` and is used to render all blog posts on the blog page.
#[derive(Clone, Debug, Deserialize)]
pub struct AllPosts {
    /// A list containing all blog posts.
    pub posts: Vec<PostData>,
}

/// This struct holds post data sent from the API.
#[derive(Clone, Debug, Deserialize)]
pub struct PostData {
    /// The body of the post.
    pub body: String,
    /// The created timestamp.
    pub created: String,
    /// The edited timestamp.
    pub edited: Option<String>,
    /// The post's ID.
    pub post_id: String,
    /// The preview/main image link.
    pub preview_image_link: String,
    /// The preview summary.
    pub preview_summary: String,
    /// The title of the post.
    pub title: String,
    /// The topic of the post.
    pub topic: String,
    /// The view count.
    pub view_count: i32,
}

impl PostData {
    /// Create a new `PostData` given the post's `post_id`.
    pub fn from_post_id(post_id: String) -> Self {
        Self {
            body: "".to_string(),
            created: "".to_string(),
            edited: None,
            post_id,
            preview_image_link: "".to_string(),
            preview_summary: "".to_string(),
            title: "".to_string(),
            topic: "".to_string(),
            view_count: 0,
        }
    }
}
