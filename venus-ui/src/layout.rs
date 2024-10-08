use leptos::*;

use crate::clsx;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! { <main class=clsx!("flex")>{children().nodes.into_iter().collect::<Vec<_>>()}</main> }
}
