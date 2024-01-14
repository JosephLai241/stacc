//! Contains the trait and its implementations for sorting `HashMap<String, i32>`s by their values.

use std::collections::HashMap;

use crate::models::chicago::{CleanedShotData, CleanedViolenceData};

/// This trait enables a struct to sort all of its `HashMap` fields in descending order assuming its
/// `HashMap` field is of type `HashMap<String, i32>`.
pub trait SortableStruct {
    /// Sort all HashMaps in this struct of type `HashMap<String, i32>` by its value (descending).
    fn sort_hashmaps(&mut self) {}

    /// The sorting algorithm for sorting a single `HashMap<String, i32>` in descending order.
    fn sort_single_hashmap(
        &mut self,
        unsorted_hashmap: &mut HashMap<String, i32>,
    ) -> HashMap<String, i32> {
        let mut sorted_values: Vec<(&String, &i32)> = unsorted_hashmap.iter().collect();
        sorted_values.sort_by(|first_tuple, second_tuple| second_tuple.1.cmp(first_tuple.1));

        let sorted_hashmap: HashMap<String, i32> = sorted_values
            .into_iter()
            .map(|(key, value)| (key.clone(), *value))
            .collect();

        sorted_hashmap
    }
}

impl SortableStruct for CleanedShotData {
    fn sort_hashmaps(&mut self) {
        unimplemented!()
    }
}

impl SortableStruct for CleanedViolenceData {
    fn sort_hashmaps(&mut self) {
        unimplemented!()
    }
}
