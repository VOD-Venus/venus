use crate::{
    clsx,
    components::{dark_mode_btn::DarkMode, errors::ErrorsView},
};
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorsView errors=errors /> }
        }>

            <div>
                <h1>"Welcome to Leptos"</h1>

                <div class="p-2">
                    <button on:click=move |_| set_count(count() + 1) class=clsx!("btn mr-2")>
                        {count}
                    </button>
                    <button class=clsx!("btn mr-2")>{count}</button>
                    <DarkMode />
                </div>
            </div>
        </ErrorBoundary>
    }
}
