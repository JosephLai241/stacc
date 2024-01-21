//! Contains all routes for the site.

use yew_router::prelude::*;

/// Contains all route variants for the site.
#[derive(Clone, Debug, Eq, PartialEq, Routable)]
pub enum Route {
    /// About page.
    #[at("/about")]
    About,
    /// Blog page.
    #[at("/blog")]
    Blog,
    /// 404 page for fuckers tryna dox me or some shit.
    #[not_found]
    #[at("/404")]
    NotFound,
    /// Single post view.
    #[at("/blog/post/:post_id")]
    PostView { post_id: String },
    /// Root page (landing page).
    #[at("/")]
    Root,
    #[at("/violence")]
    Violence,
}
