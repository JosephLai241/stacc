//! Contains the trait and its implementations for creating map marker popups (the `violence`
//! page).

use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::America::Chicago;
use sha2::{Digest, Sha256};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    Document, Element, HtmlElement, HtmlTableCellElement, HtmlTableElement, HtmlTableRowElement,
};

use crate::{
    errors::StaccError,
    models::chicago::{ShotData, ViolenceData},
};

/// This trait enables the data for a given struct to be converted into an HTML popup by
/// constructing a `JsValue`.
pub trait Popup {
    /// Neatly display the data found in the struct in a popup.
    fn into_popup(&self) -> Result<JsValue, StaccError>;

    /// Create the header for the popup.
    fn create_popup_header(
        &self,
        document: &Document,
        popup_content: &Element,
    ) -> Result<(), StaccError>;

    /// Create the metadata subtitle/secondary header for the popup.
    fn create_metadata_subtitle(
        &self,
        document: &Document,
        popup_content: &Element,
        timestamp: String,
    ) -> Result<(), StaccError>;

    /// Create the info table containing miscellaneous incident information.
    fn create_info_table(
        &self,
        document: &Document,
        popup_content: &Element,
    ) -> Result<(), StaccError>;

    /// Build a new row for the info table.
    fn build_table_row(
        &self,
        document: &Document,
        row_title: &str,
        row_value: &str,
    ) -> Result<HtmlTableRowElement, StaccError> {
        let row = document
            .create_element("tr")?
            .dyn_into::<HtmlTableRowElement>()?;

        let title = document
            .create_element("td")?
            .dyn_into::<HtmlTableCellElement>()?;
        title.set_class_name("marker-popup-table-cell");
        title.set_inner_html(&format!("<b>{row_title}</b>"));

        let value = document
            .create_element("td")?
            .dyn_into::<HtmlTableCellElement>()?;
        value.set_class_name("marker-popup-table-cell");
        value.set_inner_html(&row_value);

        row.append_child(&title)?;
        row.append_child(&value)?;

        Ok(row)
    }

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

    fn create_popup_header(
        &self,
        document: &Document,
        popup_content: &Element,
    ) -> Result<(), StaccError> {
        let header = document.create_element("h5")?;
        header.set_inner_html(&self.incident_type_description);
        popup_content.append_child(&header)?;

        Ok(())
    }

    fn create_metadata_subtitle(
        &self,
        document: &Document,
        popup_content: &Element,
        timestamp: String,
    ) -> Result<(), StaccError> {
        let meta_subtitle = document.create_element("small")?;
        meta_subtitle.set_inner_html(&format!(
            "{} | {} {}",
            timestamp, self.community_area, self.zip_code
        ));
        popup_content.append_child(&meta_subtitle)?;

        Ok(())
    }

    fn create_info_table(
        &self,
        document: &Document,
        popup_content: &Element,
    ) -> Result<(), StaccError> {
        let info_table = document
            .create_element("table")?
            .dyn_into::<HtmlTableElement>()?;
        info_table.set_class_name("marker-popup-table");

        let block_row =
            self.build_table_row(document, "Block", &self.block.trim_end_matches(','))?;
        info_table.append_child(&block_row)?;

        if self.incident_type_description.to_lowercase() == "multiple gunshots" {
            let rounds_row = self.build_table_row(document, "Rounds", &self.rounds)?;
            info_table.append_child(&rounds_row)?;
        }

        popup_content.append_child(&info_table.dyn_into::<HtmlElement>()?.into())?;

        Ok(())
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

    fn create_popup_header(
        &self,
        document: &Document,
        popup_content: &Element,
    ) -> Result<(), StaccError> {
        let crime_description = self.get_crime_description();

        let header = document.create_element("h5")?;
        header.set_inner_html(&crime_description);
        popup_content.append_child(&header)?;

        Ok(())
    }

    fn create_metadata_subtitle(
        &self,
        document: &Document,
        popup_content: &Element,
        timestamp: String,
    ) -> Result<(), StaccError> {
        let meta_subtitle = document.create_element("small")?;
        meta_subtitle.set_inner_html(&format!(
            "{} | {} {}",
            timestamp, self.community_area, self.zip_code
        ));
        popup_content.append_child(&meta_subtitle)?;

        Ok(())
    }

    fn create_info_table(
        &self,
        document: &Document,
        popup_content: &Element,
    ) -> Result<(), StaccError> {
        let info_table = document
            .create_element("table")?
            .dyn_into::<HtmlTableElement>()?;
        info_table.set_class_name("marker-popup-table");

        let gunshot_injury_row =
            self.build_table_row(document, "Gunshot injury?", &self.gunshot_injury_i)?;
        info_table.append_child(&gunshot_injury_row)?;

        let location_description_row =
            self.build_table_row(document, "Location", &self.location_description)?;
        info_table.append_child(&location_description_row)?;

        let race_row = self.build_table_row(document, "Victim race", &self.race)?;
        info_table.append_child(&race_row)?;

        let age_row = self.build_table_row(document, "Victim age", &self.age)?;
        info_table.append_child(&age_row)?;

        let sex_row = self.build_table_row(document, "Victim sex", &self.sex)?;
        info_table.append_child(&sex_row)?;

        popup_content.append_child(&info_table.dyn_into::<HtmlElement>()?.into())?;

        Ok(())
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
