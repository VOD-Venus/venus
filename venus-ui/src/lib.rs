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

#[derive(Debug, Clone, Copy)]
struct Tabs<'a> {
    /// 首页的 Tab 标签页 ID
    pub home: &'a str,
}
#[derive(Copy, Clone, Debug)]
struct GlobalUI {
    /// 各个页面标签页的 tab index
    pub tabs: RwSignal<Tabs<'static>>,
}
impl GlobalUI {
    pub fn new() -> Self {
        Self {
            tabs: create_rw_signal(Tabs {
                home: "subscription",
            }),
        }
    }
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

    // ui 的全局状态
    provide_context(GlobalUI::new());

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" />

        <Title text="Venus" />

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
