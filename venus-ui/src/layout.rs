use leptos::*;

use crate::components::{
    errors::ErrorsView,
    sidebar::{Sidebar, SidebarMobile},
};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorsView errors=errors /> }
        }>
            <main class="flex h-full">
                <Sidebar />

                <div class="flex-1 h-full p-8 overflow-auto">
                    <SidebarMobile />
                    {children().nodes.into_iter().collect::<Vec<_>>()}
                </div>
            </main>
        </ErrorBoundary>
    }
}
