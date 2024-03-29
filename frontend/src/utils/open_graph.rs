//! Contains utilities pertaining to Open Graph tags.

use gloo_utils::document;

use crate::errors::StaccError;

/// Contains all page types that may be set in the Open Graph `og:type` tag for this particular
/// site.
#[derive(Debug)]
pub enum PageType {
    /// The current page contains an article.
    Article,
    /// This variant is used for all other pages on the site that do not include an article.
    Website,
}

impl ToString for PageType {
    fn to_string(&self) -> String {
        match self {
            PageType::Article => "article".to_string(),
            PageType::Website => "website".to_string(),
        }
    }
}

/// Contains all Open Graph tags to set on the page.
#[derive(Debug)]
pub enum OpenGraphTag {
    /// Set the Open Graph description tag.
    Description(String),
    /// Set the Open Graph image tag.
    ImageLink(String),
    /// Set the Open Graph type tag.
    PageType(PageType),
    /// Set the Open Graph title tag.
    Title(String),
    /// Set the Open Graph URL tag.
    Url(String),
}

impl OpenGraphTag {
    /// Convert the enum variant to a tuple containing the tag's property and content attributes.
    pub fn to_tuple(&self) -> (String, String) {
        match self {
            OpenGraphTag::Description(description) => {
                ("og:description".to_string(), description.clone())
            }
            OpenGraphTag::ImageLink(link) => ("og:image".to_string(), link.clone()),
            OpenGraphTag::PageType(page_type) => {
                let content = page_type.to_string();

                ("og:type".to_string(), content)
            }
            OpenGraphTag::Title(title) => ("og:title".to_string(), title.clone()),
            OpenGraphTag::Url(url) => ("og:url".to_string(), url.clone()),
        }
    }
}

/// Set an Open Graph tag for a particular page.
pub fn set_open_graph_tag(og_tag: OpenGraphTag) -> Result<(), StaccError> {
    let (property, content) = og_tag.to_tuple();

    let new_tag = document().create_element("meta")?;
    new_tag.set_attribute("property", &property)?;
    new_tag.set_attribute("content", &content)?;

    if let Some(head) = document().head() {
        head.append_child(&new_tag)?;
    }

    Ok(())
}
