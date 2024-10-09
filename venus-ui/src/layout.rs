use leptos::*;

use crate::{
    clsx,
    components::{errors::ErrorsView, sidebar::Sidebar},
};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorsView errors=errors /> }
        }>
            <main class=clsx!("flex h-full")>
                <Sidebar />

                <div class="p-8 flex-1 overflow-auto h-full">
                    {children().nodes.into_iter().collect::<Vec<_>>()}
                </div>
            </main>
        </ErrorBoundary>
    }
}
