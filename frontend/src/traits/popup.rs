//! Contains the trait and its implementations for creating map marker popups (the `violence`
//! page).

use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::America::Chicago;
use sha2::{Digest, Sha256};
use wasm_bindgen::JsValue;

use crate::{
    errors::StaccError,
    models::chicago::{ShotData, ViolenceData},
};

/// This trait enables the data for a given struct to be converted into an HTML popup by
/// constructing a `JsValue`.
pub trait Popup {
    /// Neatly display the data found in the struct in a popup.
    fn into_popup(&self) -> Result<JsValue, StaccError>;

    /// Generate a SHA256 ID for this particular struct. This ID is referenced when constructing a
    /// popup HTML element by referencing this value in the element's `id` attribute.
    fn generate_id(&self) -> String;

    /// Format the date to a more human-readable format.
    fn format_date(&self) -> String;
}

impl Popup for ShotData {
    fn into_popup(&self) -> Result<JsValue, StaccError> {
        let hash_id = self.generate_id();
        let timestamp = self.format_date();

        let document = gloo_utils::document();

        let popup = document.create_element("div")?;
        popup.set_id(&format!("shot-data-{hash_id}-popup"));

        let popup_content = document.create_element("div")?;

        self.create_popup_header(&document, &popup_content)?;
        self.create_metadata_subtitle(&document, &popup_content, timestamp)?;
        self.create_info_table(&document, &popup_content)?;

        popup.append_child(&popup_content)?;

        Ok(JsValue::from(popup))
    }

    fn generate_id(&self) -> String {
        let mut hash_string = self.block.to_string();
        hash_string.push_str(&self.community_area);
        hash_string.push_str(&self.date);
        hash_string.push_str(&self.incident_type_description);
        hash_string.push_str(&format!("{:?}", self.location));
        hash_string.push_str(&self.rounds);
        hash_string.push_str(&self.zip_code);

        let mut hasher = Sha256::new();
        hasher.update(hash_string.as_bytes());

        let hash_result = hasher.finalize();

        hex::encode(hash_result)
    }

    fn format_date(&self) -> String {
        let parsed_datetime = NaiveDateTime::parse_from_str(&self.date, "%Y-%m-%dT%H:%M:%S%.3f")
            .ok()
            .map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc));

        let chicago_datetime = parsed_datetime.map(|datetime| datetime.with_timezone(&Chicago));

        let formatted_str =
            chicago_datetime.map(|datetime| datetime.format("%Y/%m/%d %H:%M:%S %Z").to_string());

        if let Some(formatted_date) = formatted_str {
            formatted_date
        } else {
            self.date.clone()
        }
    }
}

impl Popup for ViolenceData {
    fn into_popup(&self) -> Result<JsValue, StaccError> {
        let hash_id = self.generate_id();
        let timestamp = self.format_date();
        let crime_description = self.get_crime_description();

        let document = gloo_utils::document();

        let popup = document.create_element("div")?;
        popup.set_id(&format!("shot-data-{hash_id}-popup"));

        let popup_content = document.create_element("div")?;

        self.create_popup_header(crime_description, &document, &popup_content)?;
        self.create_metadata_subtitle(&document, &popup_content, timestamp)?;
        self.create_info_table(&document, &popup_content)?;

        popup.append_child(&popup_content)?;

        Ok(JsValue::from(popup))
    }

    fn generate_id(&self) -> String {
        let mut hash_string = self.age.to_string();
        hash_string.push_str(&self.community_area);
        hash_string.push_str(&self.date);
        hash_string.push_str(&self.gunshot_injury_i);
        hash_string.push_str(&self.incident_iucr_cd);
        hash_string.push_str(&self.incident_primary);
        hash_string.push_str(&format!("{:?}", self.location));
        hash_string.push_str(&self.location_description);
        hash_string.push_str(&self.race);
        hash_string.push_str(&self.sex);
        hash_string.push_str(&self.victimization_fbi_cd);
        hash_string.push_str(&self.victimization_fbi_descr);
        hash_string.push_str(&self.zip_code);

        let mut hasher = Sha256::new();
        hasher.update(hash_string.as_bytes());

        let hash_result = hasher.finalize();

        hex::encode(hash_result)
    }

    fn format_date(&self) -> String {
        let parsed_datetime = NaiveDateTime::parse_from_str(&self.date, "%Y-%m-%dT%H:%M:%S%.3f")
            .ok()
            .map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc));

        let chicago_datetime = parsed_datetime.map(|datetime| datetime.with_timezone(&Chicago));

        let formatted_str =
            chicago_datetime.map(|datetime| datetime.format("%Y/%m/%d %H:%M:%S %Z").to_string());

        if let Some(formatted_date) = formatted_str {
            formatted_date
        } else {
            self.date.clone()
        }
    }
}
