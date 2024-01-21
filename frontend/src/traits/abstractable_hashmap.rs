//! Contains the trait and its implementations for sorting `HashMap<String, i32>`s by their values.

use crate::{
    errors::StaccError,
    models::chicago::{CleanedShotData, CleanedViolenceData},
};

/// This trait enables a struct to sort all of its `HashMap` fields in descending order assuming its
/// `HashMap` field is of type `HashMap<String, i32>`.
/// Additionally, `HashMap`s may be converted into a sorted `Vec<(String, i32)>` representation.
/// This `Vec` is sorted based on the values in the `HashMap` before conversion.
pub trait AbstractableHashMap {
    /// Insert a new key/value pair, or increment an existing value of a given key.
    fn insert_or_increment(&mut self, hashmap_name: &str, key: &str) -> Result<(), StaccError>;
    /// Convert a `HashMap<String, i32>` to a sorted `Vec<(String, i32)>` based on the values in
    /// the `HashMap`.
    fn to_vec(&self, hashmap_name: &str) -> Result<Vec<(String, i32)>, StaccError>;
}

impl AbstractableHashMap for CleanedShotData {
    fn insert_or_increment(&mut self, hashmap_name: &str, key: &str) -> Result<(), StaccError> {
        let hashmap = match hashmap_name {
            "sorted_blocks" => Ok(&mut self.sorted_blocks),
            "sorted_community_areas" => Ok(&mut self.sorted_community_areas),
            "sorted_incident_types" => Ok(&mut self.sorted_incident_types),
            "sorted_rounds" => Ok(&mut self.sorted_rounds),
            "sorted_zip_codes" => Ok(&mut self.sorted_zip_codes),
            _ => Err(StaccError::InvalidHashMapError(hashmap_name.to_string())),
        }?;

        let entry = hashmap.entry(key.to_string()).or_insert(0);
        *entry += 1;

        Ok(())
    }

    fn to_vec(&self, hashmap_name: &str) -> Result<Vec<(String, i32)>, StaccError> {
        let hashmap = match hashmap_name {
            "sorted_blocks" => Ok(&self.sorted_blocks),
            "sorted_community_areas" => Ok(&self.sorted_community_areas),
            "sorted_incident_types" => Ok(&self.sorted_incident_types),
            "sorted_rounds" => Ok(&self.sorted_rounds),
            "sorted_zip_codes" => Ok(&self.sorted_zip_codes),
            _ => Err(StaccError::InvalidHashMapError(hashmap_name.to_string())),
        }?;

        let mut sorted_vec: Vec<(String, i32)> = hashmap
            .iter()
            .map(|(key, value)| (key.clone(), *value))
            .collect();
        sorted_vec.sort_by(|a, b| a.1.cmp(&b.1));
        sorted_vec.reverse();

        Ok(sorted_vec)
    }
}

impl AbstractableHashMap for CleanedViolenceData {
    fn insert_or_increment(&mut self, hashmap_name: &str, key: &str) -> Result<(), StaccError> {
        let hashmap = match hashmap_name {
            "sorted_ages" => Ok(&mut self.sorted_ages),
            "sorted_community_areas" => Ok(&mut self.sorted_community_areas),
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

    fn to_vec(&self, hashmap_name: &str) -> Result<Vec<(String, i32)>, StaccError> {
        let hashmap = match hashmap_name {
            "sorted_ages" => Ok(&self.sorted_ages),
            "sorted_community_areas" => Ok(&self.sorted_community_areas),
            "sorted_gun_injury_count" => Ok(&self.sorted_gun_injury_count),
            "sorted_incident_types" => Ok(&self.sorted_incident_types),
            "sorted_location_descriptions" => Ok(&self.sorted_location_descriptions),
            "sorted_victim_races" => Ok(&self.sorted_victim_races),
            "sorted_victim_sexes" => Ok(&self.sorted_victim_sexes),
            "sorted_zip_codes" => Ok(&self.sorted_zip_codes),
            _ => Err(StaccError::InvalidHashMapError(hashmap_name.to_string())),
        }?;

        let mut sorted_vec: Vec<(String, i32)> = hashmap
            .iter()
            .map(|(key, value)| (key.clone(), *value))
            .collect();
        sorted_vec.sort_by(|a, b| a.1.cmp(&b.1));
        sorted_vec.reverse();

        Ok(sorted_vec)
    }
}
