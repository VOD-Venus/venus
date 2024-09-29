use leptos::*;

#[component]
pub fn ErrorsView(errors: RwSignal<Errors>) -> impl IntoView {
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
}
