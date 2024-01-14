//! Contains the trait and its implementations for sorting `HashMap<String, i32>`s by their values.

use crate::{
    errors::StaccError,
    models::chicago::{CleanedShotData, CleanedViolenceData},
};

/// This trait enables a struct to sort all of its `HashMap` fields in descending order assuming its
/// `HashMap` field is of type `HashMap<String, i32>`.
pub trait AbstractableHashMap {
    /// Insert a new key/value pair, or increment an existing value of a given key.
    fn insert_or_increment(&mut self, hashmap_name: &str, key: &str) -> Result<(), StaccError>;
}

impl AbstractableHashMap for CleanedShotData {
    fn insert_or_increment(&mut self, hashmap_name: &str, key: &str) -> Result<(), StaccError> {
        let hashmap = match hashmap_name {
            "sorted_blocks" => Ok(&mut self.sorted_blocks),
            "sorted_community_areas" => Ok(&mut self.sorted_community_areas),
            "sorted_dates" => Ok(&mut self.sorted_dates),
            "sorted_incident_types" => Ok(&mut self.sorted_incident_types),
            "sorted_rounds" => Ok(&mut self.sorted_rounds),
            "sorted_zip_codes" => Ok(&mut self.sorted_zip_codes),
            _ => Err(StaccError::InvalidHashMapError(hashmap_name.to_string())),
        }?;

        let entry = hashmap.entry(key.to_string()).or_insert(0);
        *entry += 1;

        Ok(())
    }
}

impl AbstractableHashMap for CleanedViolenceData {
    fn insert_or_increment(&mut self, hashmap_name: &str, key: &str) -> Result<(), StaccError> {
        let hashmap = match hashmap_name {
            "sorted_ages" => Ok(&mut self.sorted_ages),
            "sorted_community_areas" => Ok(&mut self.sorted_community_areas),
            "sorted_dates" => Ok(&mut self.sorted_dates),
            "sorted_gun_injury_count" => Ok(&mut self.sorted_gun_injury_count),
            "sorted_incident_types" => Ok(&mut self.sorted_incident_types),
            "sorted_location_descriptions" => Ok(&mut self.sorted_location_descriptions),
            "sorted_victim_races" => Ok(&mut self.sorted_victim_races),
            "sorted_victim_sexes" => Ok(&mut self.sorted_victim_sexes),
            "sorted_zip_codes" => Ok(&mut self.sorted_zip_codes),
            _ => Err(StaccError::InvalidHashMapError(hashmap_name.to_string())),
        }?;

        let entry = hashmap.entry(key.to_string()).or_insert(0);
        *entry += 1;

        Ok(())
    }
}
