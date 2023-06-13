//! Instantiate a connection to MongoDB.

use mongodb::{Client, Collection};

use crate::{
    errors::StaccError,
    models::{
        data::{BackgroundGIF, Story},
        post::PostData,
        visitor::Visitor,
    },
};

use super::environment::EnvironmentVariables;

#[derive(Clone, Debug)]
pub struct Mongo {
    /// The collection containing background GIFs.
    pub backgrounds_collection: Collection<BackgroundGIF>,
    /// The collection containing posts.
    pub posts_collection: Collection<PostData>,
    /// The collection containing 404 page stories.
    pub stories_collection: Collection<Story>,
    /// The collection containing visitor information.
    pub visitor_collection: Collection<Visitor>,
}

impl Mongo {
    /// Initialize a new MongoDB client.
    pub async fn init() -> Result<Self, StaccError> {
        let mongo_uri = format!(
            "mongodb://{}:{}@{}",
            EnvironmentVariables::MongoDBUser.env_var()?,
            EnvironmentVariables::MongoDBPassword.env_var()?,
            EnvironmentVariables::MongoDBURI.env_var()?,
        );
        let client = Client::with_uri_str(&mongo_uri).await?;
        let database = client.database(&EnvironmentVariables::StaccDatabase.env_var()?);

        Ok(Self {
            backgrounds_collection: database.collection::<BackgroundGIF>(
                &EnvironmentVariables::StaccBackgroundsCollectionName.env_var()?,
            ),
            posts_collection: database
                .collection::<PostData>(&EnvironmentVariables::StaccPostsCollectionName.env_var()?),
            stories_collection: database
                .collection::<Story>(&EnvironmentVariables::StaccStoriesCollectionName.env_var()?),
            visitor_collection: database.collection::<Visitor>(
                &EnvironmentVariables::StaccVisitorsCollectionName.env_var()?,
            ),
        })
    }
}
