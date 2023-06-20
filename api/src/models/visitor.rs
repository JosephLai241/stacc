//! Contains models for visitors.

use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Contains visitor information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Visitor {
    /// The first visit date from this visitor's IP address.
    pub first_visit_date: String,
    /// The IP address associated with this visitor's machine.
    pub ip_address: String,
    /// The IP data associated with this visitor's IP address.
    pub ip_data: Option<IPData>,
    /// The last visit date from this visitor's IP address.
    pub last_visit_date: Option<String>,
    /// The approximate number of times this visitor has refreshed the site. This number is only
    /// incremented when the user has done one of the following:
    /// 1. Refreshed the background GIF.
    /// 2. Refreshed the 404 page.
    /// 3. Loaded/refreshed the blog page containing all blog posts.
    /// 4. Loaded/refreshed a particular post.
    pub refresh_count: i32,
    /// The posts that this visitor has loaded.
    /// The key corresponds with the post's ID, and the value corresponds with the number of
    /// refreshes this visitor has made for a particular post.
    pub visited_posts: HashMap<String, i32>,
}

impl Visitor {
    /// Create a new `Visitor`.
    pub fn new(ip_address: String) -> Self {
        Self {
            first_visit_date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            ip_address,
            ip_data: None,
            last_visit_date: None,
            refresh_count: 1,
            visited_posts: HashMap::new(),
        }
    }
}

/// This struct holds IP metadata returned from querying [ip-api](https://ip-api.com/) for an IP's
/// data.
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct IPData {
    /// The Autonomous System number and organization.
    /// May be empty for IP blocks not being announced in BGP tables.
    ///
    /// NOTE: The `r#` escapes `"as"`. This is necessary because `"as"` is a reserved keyword.
    pub r#as: String,
    /// The city associated with the IP address.
    pub city: String,
    /// The continent associated with the IP address.
    pub continent: String,
    /// The country associated with the IP address.
    pub country: String,
    /// The country code associated with the IP address.
    pub countryCode: String,
    /// The currency associated with the IP address.
    pub currency: String,
    /// Whether the IP address is coming from hosting, colocated, or data center.
    pub hosting: bool,
    /// The ISP associated with the IP address.
    pub isp: String,
    /// The latitude associated with the IP address.
    pub lat: f64,
    /// The longitude associated with the IP address.
    pub lon: f64,
    /// Whether the IP address is coming from a mobile device.
    pub mobile: bool,
    /// The organization associated with the IP address.
    pub org: String,
    /// Whether the IP address is using a proxy, VPN, or Tor exit address.
    pub proxy: bool,
    /// The IP address itself.
    pub query: String,
    /// The region associated with the IP address.
    pub region: String,
    /// The region name associated with the IP address.
    pub regionName: String,
    /// The reverse DNS associated with the IP address.
    ///
    /// NOTE: This may delay a response from `ip-api`.
    pub reverse: String,
    /// The status of this query. This value is always `"success"` or `"fail"`.
    pub status: String,
    /// The timezone associated with the IP address.
    pub timezone: String,
    /// The ZIP associated with the IP address.
    pub zip: String,
}
