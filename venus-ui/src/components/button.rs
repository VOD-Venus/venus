use std::borrow::Cow;

use leptos::*;

use crate::clsx;

/// A parameterized incrementing button
#[component]
pub fn Button(children: Children, #[prop(optional)] class: Cow<'static, str>) -> impl IntoView {
    view! {
        <button class=clsx!(
            "bg-white border border-transparent hover:border-gray-200",
          "outline-none hover:bg-gray-50 focus:ring-4 dark:border-transparent",
          "focus:ring-cyan-200 font-medium rounded-lg text-sm",
          "px-5 py-2.5 dark:bg-gray-800 dark:text-white ",
          "dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:ring-cyan-800",
          "transition-all disabled:hover:bg-gray-200",
          "disabled:cursor-not-allowed disabled:dark:hover:bg-gray-700",
          "disabled:hover:border-transparent",
          "text-lg disabled:bg-gray-200 disabled:text-gray-500",
          "dark:disabled:bg-gray-700 dark:disabled:text-gray-300",
          "disabled:dark:hover:border-transparent",
          class
        )>{children().nodes.into_iter().collect::<Vec<_>>()}</button>
    }
}
