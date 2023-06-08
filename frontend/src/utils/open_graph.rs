//! Contains utilities pertaining to Open Graph tags.

use gloo_utils::document;

use crate::errors::StaccError;

/// Contains all page types that may be set in the Open Graph `og:type` tag for this particular
/// site.
pub enum PageType {
    /// The current page contains an article.
    Article,
    /// This variant is used for all other pages on the site that do not include an article.
    Website,
}

/// Contains all Open Graph tags to set on the page.
pub enum OpenGraphTag {
    /// Set the Open Graph description tag.
    Description { text: String },
    /// Set the Open Graph image tag.
    ImageLink { link: String },
    /// Set the Open Graph type tag.
    PageType { page_type: PageType },
    /// Set the Open Graph title tag.
    Title { text: String },
    /// Set the Open Graph URL tag.
    Url { text: String },
}

impl OpenGraphTag {
    /// Convert the enum variant to a tuple containing the tag's property and content attributes.
    pub fn to_tuple(self) -> (String, String) {
        match self {
            OpenGraphTag::Description { text } => ("og:description".to_string(), text),
            OpenGraphTag::ImageLink { link } => ("og:image".to_string(), link),
            OpenGraphTag::PageType { page_type } => {
                let content = match page_type {
                    PageType::Article => "article".to_string(),
                    PageType::Website => "website".to_string(),
                };

                ("og:type".to_string(), content)
            }
            OpenGraphTag::Title { text } => ("og:title".to_string(), text),
            OpenGraphTag::Url { text } => ("og:url".to_string(), text),
        }
    }
}

/// Set the Open Graph tags for a particular page.
fn set_open_graph_tag(og_tag: OpenGraphTag) -> Result<(), StaccError> {
    let new_tag = document().create_element("meta")?;

    let (property, content) = og_tag.to_tuple();

    new_tag.set_attribute("property", &property)?;
    new_tag.set_attribute("content", &content)?;

    if let Some(head) = document().head() {
        head.append_child(&new_tag)?;
    }

    Ok(())
}
