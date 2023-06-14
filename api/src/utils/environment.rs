//! Contains utilities pertaining to the environment in which the API runs.

use std::env;

use crate::errors::StaccError;

/// Contains all environment variables required to run the API.
pub enum EnvironmentVariables {
    /// The MongoDB password.
    MongoDBPassword,
    /// The MongoDB URI.
    MongoDBURI,
    /// The MongoDB user.
    MongoDBUser,
    /// The port number the API runs on.
    StaccAPIPortNumber,
    /// The name of the collection that contains all backgrounds.
    StaccBackgroundsCollectionName,
    /// The name of the database in MongoDB.
    StaccDatabase,
    /// The domain of the site.
    StaccDomain,
    /// The name of the collection that contains all posts.
    StaccPostsCollectionName,
    /// The name of the collection that contains all stories.
    StaccStoriesCollectionName,
    /// The name of the collection that contains all visitors.
    StaccVisitorsCollectionName,
}

impl EnvironmentVariables {
    /// Get the environment variable associated with the provided variant.
    pub fn env_var(&self) -> Result<String, StaccError> {
        match self {
            Self::MongoDBPassword => Ok(env::var("MONGO_PASSWORD")?),
            Self::MongoDBURI => Ok(env::var("MONGO_URI")?),
            Self::MongoDBUser => Ok(env::var("MONGO_USER")?),
            Self::StaccAPIPortNumber => Ok(env::var("STACC_API_PORT_NUMBER")?),
            Self::StaccBackgroundsCollectionName => {
                Ok(env::var("STACC_BACKGROUNDS_COLLECTION_NAME")?)
            }
            Self::StaccDatabase => Ok(env::var("STACC_DATABASE")?),
            Self::StaccDomain => Ok(env::var("STACC_DOMAIN")?),
            Self::StaccPostsCollectionName => Ok(env::var("STACC_POSTS_COLLECTION_NAME")?),
            Self::StaccStoriesCollectionName => Ok(env::var("STACC_STORIES_COLLECTION_NAME")?),
            Self::StaccVisitorsCollectionName => Ok(env::var("STACC_VISITORS_COLLECTION_NAME")?),
        }
    }
}
