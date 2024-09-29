use std::borrow::Cow;

use leptos::*;

use crate::clsx;

/// 颜色模式切换下拉框
#[component]
pub fn DarkMode(#[prop(optional)] class: Cow<'static, str>) -> impl IntoView {
    view! {
        <details class=clsx!("dropdown", class)>
            <summary class="btn m-1">open or close</summary>
            <ul class="menu dropdown-content bg-base-100 rounded-box z-[1] w-52 p-2 shadow">
                <li>
                    <a>Item 1</a>
                </li>
                <li>
                    <a>Item 2</a>
                </li>
            </ul>
        </details>
    }
}
