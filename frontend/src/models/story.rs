//! Contains all models pertaining to stories.

use serde::Deserialize;

/// The story to display on the 404 not found page.
#[derive(Clone, Debug, Deserialize)]
pub struct Story {
    /// The body of the story.
    pub story: String,
}

impl Default for Story {
    fn default() -> Self {
        Self {
            story:
                "If you don’t like the road you’re walking, pave another one. Except for this one."
                    .to_string(),
        }
    }
}
