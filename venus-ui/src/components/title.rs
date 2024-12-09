use leptos::prelude::*;

#[component]
pub fn Title(children: Children) -> impl IntoView {
    view! { <h1 class="mt-1 mb-4 text-4xl dark:text-gray-300">{children()}</h1> }
}
