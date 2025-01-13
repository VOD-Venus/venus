#![feature(stmt_expr_attributes)]
use components::notifications::Notifications;
use consts::{COLOR_MODE, SIDEBAR_OPEN_KEY, TABS_KEY, USER_KEY};
use gloo::storage::{LocalStorage, Storage};
use hooks::use_global_ui;
use layout::Layout;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use leptos_use::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};
use pages::{
    about::About, dashboard::Dashboard, editor::Editor, logging::Logging, login::Login,
    not_found::NotFound, proxies::Proxies, settings::Settings,
};
use serde::{Deserialize, Serialize};
use utils::nanoid;

mod api;
mod components;
mod consts;
mod hooks;
mod layout;
mod pages;
mod utils;

/// 各个页面的保存的 Tab ID，用于持久化 Tab 状态
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tabs {
    /// 代理页的 Tab 标签页 ID
    pub proxies: String,
}
impl Tabs {
    pub fn new() -> Self {
        let tabs = LocalStorage::get::<Tabs>(TABS_KEY).unwrap_or_default();
        Self { ..tabs }
    }
}
impl Default for Tabs {
    fn default() -> Self {
        Self {
            proxies: "subscription".into(),
        }
    }
}

/// 通知类型
#[derive(Debug, Clone)]
pub enum NotificationKind {
    Success,
    Info,
    Warning,
    Error,
}
/// 通知消息，由于右上角通知栏
#[derive(Debug, Clone)]
pub struct Notification {
    pub key: String,
    pub kind: NotificationKind,
    pub message: String,
}
impl Notification {
    pub fn new(kind: NotificationKind, message: String) -> Self {
        Self {
            key: nanoid(6),
            kind,
            message,
        }
    }
}

/// 登录后保存的用户信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    pub server: String,
    pub username: String,
    /// 用户的 jwt token
    /// 决定了用户是否已经登录
    pub token: String,
    pub token_type: String,
}
impl User {
    pub fn new() -> Self {
        let user = LocalStorage::get::<User>(USER_KEY).unwrap_or_default();
        Self { ..user }
    }
}

#[derive(Copy, Clone, Debug)]
struct GlobalUI {
    /// 各个页面标签页的 tab index，保存到 localStorage
    pub tabs: RwSignal<Tabs>,
    /// 整个 App 的通知 右上角
    pub notifications: RwSignal<Vec<Notification>>,
    /// 用户信息，保存到 localStorage
    pub user: RwSignal<User>,
    /// Sidebar 的打开状态，保存到 localStorage
    pub sidebar_open: RwSignal<bool>,
}
impl GlobalUI {
    pub fn new() -> Self {
        let sidebar_open = LocalStorage::get::<bool>(SIDEBAR_OPEN_KEY).unwrap_or_default();

        Self {
            tabs: RwSignal::new(Tabs::new()),
            notifications: RwSignal::new(vec![]),
            user: RwSignal::new(User::new()),
            sidebar_open: RwSignal::new(sidebar_open),
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
    let global_ui = GlobalUI::new();
    provide_context(global_ui);
    // persist ui
    Effect::new(|| {
        let ui = use_global_ui();
        let tab = ui.tabs.get();
        LocalStorage::set(TABS_KEY, tab).ok();
    });
    Effect::new(|| {
        let ui = use_global_ui();
        let user = ui.user.get();
        LocalStorage::set(USER_KEY, user).ok();
    });
    Effect::new(|| {
        let ui = use_global_ui();
        let sidebar_open = ui.sidebar_open.get();
        LocalStorage::set(SIDEBAR_OPEN_KEY, sidebar_open).ok();
    });

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let logged_in = Memo::new(move |_| Some(!global_ui.user.read().token.is_empty()));
    let redirect_path = || "/login";

    view! {
        <Html {..} lang="en" dir="ltr" />
        <Title text="Venus" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=NotFound>
                <ParentRoute path=path!("/") view=Layout>
                    <ProtectedRoute
                        path=path!("/")
                        view=Dashboard
                        condition=move || logged_in.get()
                        redirect_path=redirect_path
                    />
                    <ProtectedRoute
                        path=path!("/dashboard")
                        view=Dashboard
                        condition=move || logged_in.get()
                        redirect_path=redirect_path
                    />
                    <ProtectedRoute
                        path=path!("/proxies")
                        view=Proxies
                        condition=move || logged_in.get()
                        redirect_path=redirect_path
                    />
                    <ProtectedRoute
                        path=path!("/settings")
                        view=Settings
                        condition=move || logged_in.get()
                        redirect_path=redirect_path
                    />
                    <ProtectedRoute
                        path=path!("/logging")
                        view=Logging
                        condition=move || logged_in.get()
                        redirect_path=redirect_path
                    />
                    <ProtectedRoute
                        path=path!("/editor")
                        view=Editor
                        condition=move || logged_in.get()
                        redirect_path=redirect_path
                    />
                    <ProtectedRoute
                        path=path!("/about")
                        view=About
                        condition=move || logged_in.get()
                        redirect_path=redirect_path
                    />
                    <Route path=path!("/login") view=Login />
                    <Route path=path!("/*") view=NotFound />
                </ParentRoute>
            </Routes>
        </Router>

        // Global notifications
        <Notifications />
    }
}
