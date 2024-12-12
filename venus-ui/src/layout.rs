use leptos::{logging, prelude::*};
use leptos_router::{components::Outlet, hooks::use_location};

use crate::components::{
    errors::ErrorsView,
    sidebar::{Sidebar, SidebarMobile},
};
use crate::GlobalUI;

#[component]
pub fn Layout() -> impl IntoView {
    let user = use_context::<GlobalUI>()
        .expect("GlobalUI state is not set")
        .user;
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
