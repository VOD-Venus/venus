use crate::components::counter_btn::Button;
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div>
                <h1>"Welcome to Leptos"</h1>

                <div class="p-2">
                    <Button class="mr-2".into() />
                    <Button increment=5 />
                </div>
            </div>
        </ErrorBoundary>
    }
}
