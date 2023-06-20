//! Contains functionality pertaining to API middleware.

use actix_web::{web::Data, HttpRequest};
use chrono::Utc;
use lazy_static::lazy_static;
use log::{info, warn};
use mongodb::{
    bson::doc,
    options::{FindOneAndUpdateOptions, ReturnDocument},
};

use crate::{
    errors::StaccError,
    models::visitor::{IPData, Visitor},
    utils::mongo::Mongo,
};

lazy_static! {
    /// All fields that will be included in the `ip-api.com` response.
    static ref IP_API_FIELDS: Vec<&'static str> = vec![
        "as",
        "city",
        "continent",
        "country",
        "countryCode",
        "currency",
        "hosting",
        "isp",
        "lat",
        "lon",
        "message",
        "mobile",
        "org",
        "proxy",
        "query",
        "region",
        "regionName",
        "reverse",
        "status",
        "timezone",
        "zip",
    ];
}

/// Extract the real IP address from the `HttpRequest`.
fn get_real_ip(request: &HttpRequest) -> Option<String> {
    request
        .connection_info()
        .realip_remote_addr()
        .map(|real_ip| {
            let raw_ip = real_ip.to_string();
            match raw_ip.find(':') {
                Some(index) => {
                    let (ip, _port_number) = raw_ip.split_at(index);

                    ip.to_string()
                }
                None => raw_ip,
            }
        })
}

/// Add or increment the post and refresh count within the visitor's `visited_posts` `HashMap`.
pub async fn log_post_view(
    mongo: &Data<Mongo>,
    post_id: &str,
    request: &HttpRequest,
) -> Result<(), StaccError> {
    mongo
        .posts_collection
        .find_one_and_update(
            doc! { "post_id": post_id },
            doc! { "$inc": { "view_count": 1 } },
            None,
        )
        .await?;

    if let Some(ip_address) = get_real_ip(request) {
        mongo
            .visitor_collection
            .find_one_and_update(
                doc! { "ip_address": ip_address.to_string() },
                doc! { "$inc": { format!("visited_posts.{post_id}"): 1 } },
                FindOneAndUpdateOptions::builder().upsert(true).build(),
            )
            .await?;
    } else {
        warn!("FAILED TO GRAB POST VISITOR'S IP ADDRESS!");
        warn!("VISITED POST ID: {post_id}");
    }

    Ok(())
}

/// Log visitors into MongoDB. Increment the identity's `refresh_count` if the identity already
/// exists. Otherwise, grab the visitor's IP metadata and log the new visitor.
pub async fn log_visitor_data(
    mongo: &Data<Mongo>,
    request: &HttpRequest,
) -> Result<(), StaccError> {
    if let Some(ip_address) = get_real_ip(request) {
        if mongo
            .visitor_collection
            .find_one_and_update(
                doc! { "ip_address": &ip_address },
                doc! {
                    "$inc": { "refresh_count": 1 },
                    "$set": {
                        "last_visit_date": Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
                    }
                },
                FindOneAndUpdateOptions::builder()
                    .return_document(ReturnDocument::After)
                    .build(),
            )
            .await?
            .is_none()
        {
            // Grab IP data and create a new visitor in the visitors collection if this identity
            // has not already been recorded.
            let ip_data = get_ip_data(&ip_address).await.ok();

            let mut visitor = Visitor::new(ip_address.to_string());
            visitor.ip_data = ip_data;

            return mongo
                .visitor_collection
                .insert_one(visitor, None)
                .await
                .map_or_else(|error| Err(StaccError::MongoDB(error)), |_| Ok(()));
        }
    } else {
        warn!("FAILED TO GRAB VISITOR'S IP ADDRESS!");
    }

    Ok(())
}

/// Query `ip-api.com` for IP metadata.
async fn get_ip_data(ip: &str) -> Result<IPData, StaccError> {
    info!("👀 LOGGING A NEW VISITOR: {}", ip);

    let endpoint = "http://ip-api.com/json/";

    let request_endpoint = &format!("{endpoint}{ip}?fields={}", IP_API_FIELDS.join(","));
    info!("📤 SENDING REQUEST TO IP-API: {}", request_endpoint);

    let ip_data = reqwest::get(request_endpoint)
        .await?
        .json::<IPData>()
        .await?;

    info!("📥 IP DATA RECEIVED FOR IP ADDRESS: {}", ip);

    Ok(ip_data)
}
