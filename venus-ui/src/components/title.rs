use leptos::*;

use crate::clsx;

#[component]
pub fn Title(children: Children) -> impl IntoView {
    view! { <h1 class=clsx!("text-4xl dark:text-gray-300 mt-1 mb-4")>{children()}</h1> }
}
