#![feature(stmt_expr_attributes)]
use consts::COLOR_MODE;
use layout::Layout;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};
use pages::home::Home;
use pages::not_found::NotFound;
use pages::settings::Settings;

mod components;
mod consts;
mod layout;
mod pages;
mod utils;

struct Tabs {
    pub home: String,
}
#[derive(Copy, Clone, Debug)]
struct GlobalUI {
    pub tabs: RwSignal<Tabs>,
}

#[component]
pub fn App() -> impl IntoView {
    // color mode global context
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .emit_auto(true)
            .attribute("data-theme")
            .custom_modes(COLOR_MODE.iter().map(|m| m.to_string()).collect::<_>()),
    );
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
            <Layout>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/settings" view=Settings />
                    <Route path="/*" view=NotFound />
                </Routes>
            </Layout>
        </Router>
    }
}
