use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::{use_location, use_navigate};

use crate::components::{
    errors::ErrorsView,
    sidebar::{Sidebar, SidebarMobile},
};

#[component]
pub fn Layout() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();

    let logged_in = false;
    if !logged_in {
        navigate("/login", Default::default());
    } else if location.pathname.get() == "/" || location.pathname.get() == "" {
        navigate("/home", Default::default());
    }

    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorsView errors=errors.into() /> }
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
