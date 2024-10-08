use leptos::*;

use crate::clsx;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <nav class=clsx!(
            "w-56 max-w-xs flex",
            "py-6 px-5",
            "bg-gray-100 flex-col",
            "dark:bg-rua-gray-900"
        )>test</nav>
    }
}
