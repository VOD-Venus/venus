#![feature(stmt_expr_attributes)]
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod consts;
mod pages;
mod utils;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    let (current_theme, set_current_theme) = create_signal("light");
    provide_context((current_theme, set_current_theme));

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme=current_theme />

        // sets the document title
        <Title text="Venus" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/*" view=NotFound />
            </Routes>
        </Router>
    }
}
