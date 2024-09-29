#![feature(stmt_expr_attributes)]
use consts::COLOR_MODE;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};

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
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .emit_auto(true)
            .attribute("data-theme")
            .custom_modes(COLOR_MODE.iter().map(|m| m.to_string()).collect::<_>()),
    );
    logging::log!("lib {}", mode.get());
    provide_context((mode, set_mode));

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" />

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
