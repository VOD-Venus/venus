use leptos::*;

use crate::clsx;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! { <main class=clsx!("flex h-full")>{children().nodes.into_iter().collect::<Vec<_>>()}</main> }
}
