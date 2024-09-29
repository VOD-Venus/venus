use crate::components::{button::Button, errors::ErrorsView};
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
                    <Button on:click=move |_| set_count(count() + 1) class="mr-2".into()>
                        {count}
                    </Button>
                // <Button>{count}</Button>
                </div>
            </div>
        </ErrorBoundary>
    }
}
