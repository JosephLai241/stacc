//! Contains miscellaneous routes for the API.

use actix_web::{get, web::Data, HttpResponse};
use lazy_static::lazy_static;
use mongodb::{bson::doc, options::FindOneOptions};
use rand::Rng;

use crate::{errors::StaccResponseError, models::data::BackgroundGIF, utils::mongo::Mongo};

lazy_static! {
    /// The default fallback GIF if selecting a random GIF from MongoDB fails.
    /// This GIF is Takumi doing a heel-toe downshift in one of the best cars ever made -- the
    /// Subaru WRX 🥴.
    static ref FALLBACK_GIF: &'static str = "https://imgur.com/FgJDNsx.gif";
}

/// Get the background GIF by choosing a random link stored in the backgrounds collection.
#[get("/background")]
pub async fn get_background_gif(mongo: Data<Mongo>) -> Result<HttpResponse, StaccResponseError> {
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

    Ok(HttpResponse::Ok().json(background_gif))
}
