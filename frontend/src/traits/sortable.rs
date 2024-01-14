//! Contains the trait and its implementations for sorting `HashMap<String, i32>`s by their values.

use crate::models::chicago::{CleanedShotData, CleanedViolenceData};

/// This trait enables a struct to sort all of its `HashMap` fields in descending order assuming its
/// `HashMap` field is of type `HashMap<String, i32>`.
pub trait SortableStruct {
    /// Sort all HashMaps in this struct of type `HashMap<String, i32>` by its value (descending).
    fn sort_hashmaps(&mut self) {}

    /// The sorting algorithm for sorting a single `HashMap<String, i32>` in descending order.
    fn sort_single_hashmap(&mut self) {}
}

impl SortableStruct for CleanedShotData {
    fn sort_hashmaps(&mut self) {
        unimplemented!()
    }

    fn sort_single_hashmap(&mut self) {
        unimplemented!()
    }
}

impl SortableStruct for CleanedViolenceData {
    fn sort_hashmaps(&mut self) {
        unimplemented!()
    }

    fn sort_single_hashmap(&mut self) {
        unimplemented!()
    }
}
