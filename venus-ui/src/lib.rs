#![feature(stmt_expr_attributes)]
use components::notifications::Notifications;
use consts::COLOR_MODE;
use gloo::storage::{LocalStorage, Storage};
use layout::Layout;
use leptos::{logging, prelude::*};
use leptos_meta::*;
use leptos_router::{components::*, path};
use leptos_use::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};
use pages::{home::Home, login::Login, not_found::NotFound, settings::Settings};
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
#[derive(Debug, Clone, Copy)]
struct Tabs<'a> {
    /// 首页的 Tab 标签页 ID
    pub home: &'a str,
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
    pub username: String,
    /// 用户的 jwt token
    /// 决定了用户是否已经登录
    pub token: String,
    pub token_type: String,
}
impl User {
    pub fn new() -> Self {
        let user = LocalStorage::get::<User>("rua-user").unwrap_or_default();
        logging::log!("user {:?}", user);
        Self { ..user }
    }
}

#[derive(Copy, Clone, Debug)]
struct GlobalUI {
    /// 各个页面标签页的 tab index
    pub tabs: RwSignal<Tabs<'static>>,
    /// 整个 App 的通知 右上角
    pub notifications: RwSignal<Vec<Notification>>,
    /// 用户信息
    pub user: RwSignal<User>,
}
impl GlobalUI {
    pub fn new() -> Self {
        Self {
            tabs: RwSignal::new(Tabs {
                home: "subscription",
            }),
            notifications: RwSignal::new(vec![]),
            user: RwSignal::new(User::new()),
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
    logging::log!("token {:?}", global_ui.user.read().token.is_empty());

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

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
                        view=Home
                        condition=move || Some(!global_ui.user.read().token.is_empty())
                        redirect_path=|| "/login"
                    />
                    <ProtectedRoute
                        path=path!("/home")
                        view=Home
                        condition=move || Some(!global_ui.user.read().token.is_empty())
                        redirect_path=|| "/login"
                    />
                    <ProtectedRoute
                        path=path!("/settings")
                        view=Settings
                        condition=move || Some(!global_ui.user.read().token.is_empty())
                        redirect_path=|| "/login"
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
