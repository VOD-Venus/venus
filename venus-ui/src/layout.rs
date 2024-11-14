use leptos::*;
use leptos_router::{use_navigate, use_route, Outlet};

use crate::components::{
    errors::ErrorsView,
    sidebar::{Sidebar, SidebarMobile},
};

#[component]
pub fn Layout() -> impl IntoView {
    let route = use_route();
    let navigate = use_navigate();

    let logged_in = false;
    if !logged_in {
        navigate("/login", Default::default());
    } else if route.path() == "/" || route.path() == "" {
        navigate("/home", Default::default());
    }

    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorsView errors=errors /> }
        }>
            <main class="flex h-full">
                <Show when=move || logged_in fallback=|| view! { <Outlet /> }>
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
