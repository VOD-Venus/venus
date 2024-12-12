use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::{
    components::{
        errors::ErrorsView,
        sidebar::{Sidebar, SidebarMobile},
    },
    hooks::use_global_ui,
};

#[component]
pub fn Layout() -> impl IntoView {
    let user = use_global_ui().user;
    let logged_in = move || !user.read().token.is_empty();

    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorsView errors=errors.into() /> }
        }>
            <main class="flex h-full">
                <Show when=move || logged_in() fallback=|| view! { <Outlet /> }>
                    <Sidebar />
                    <div class="flex-1 h-full p-8 overflow-auto">
                        <SidebarMobile />
                        <Outlet />
                    </div>
                </Show>
            </main>
        </ErrorBoundary>
    }
}
