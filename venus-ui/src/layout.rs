use leptos::*;
use leptos_router::Outlet;

use crate::components::{
    errors::ErrorsView,
    sidebar::{Sidebar, SidebarMobile},
};

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorsView errors=errors /> }
        }>
            <main class="flex h-full">
                <Show when=|| false fallback=|| view! { <Outlet /> }>
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
