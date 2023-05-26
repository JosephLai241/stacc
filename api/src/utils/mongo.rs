//! Instantiate a connection to MongoDB.

use std::env;

use mongodb::{Client, Collection};

use crate::{
    errors::StaccError,
    models::{data::BackgroundGIF, post::PostData, visitor::Visitor},
};

#[derive(Clone, Debug)]
pub struct Mongo {
    /// The collection containing background GIFs.
    pub backgrounds_collection: Collection<BackgroundGIF>,
    /// The collection containing posts.
    pub posts_collection: Collection<PostData>,
    /// The collection containing visitor information.
    pub visitor_collection: Collection<Visitor>,
}

impl Mongo {
    /// Initialize a new MongoDB client.
    pub async fn init() -> Result<Self, StaccError> {
        let mongo_uri = format!(
            "mongodb://{}:{}@{}",
            env::var("MONGO_USER")?,
            env::var("MONGO_PASSWORD")?,
            env::var("MONGO_URI")?
        );
        let client = Client::with_uri_str(&mongo_uri).await?;
        let database = client.database(&env::var("STACC_DATABASE")?);

        Ok(Self {
            backgrounds_collection: database
                .collection::<BackgroundGIF>(&env::var("STACC_BACKGROUNDS_COLLECTION_NAME")?),
            posts_collection: database
                .collection::<PostData>(&env::var("STACC_POSTS_COLLECTION_NAME")?),
            visitor_collection: database
                .collection::<Visitor>(&env::var("STACC_VISITORS_COLLECTION_NAME")?),
        })
    }
}
