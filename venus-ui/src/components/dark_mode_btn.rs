use std::borrow::Cow;

use leptos::*;

use crate::{clsx, components::button::Button};

/// 颜色模式切换下拉框
#[component]
pub fn DarkMode(#[prop(optional)] class: Cow<'static, str>) -> impl IntoView {
    let (show, set_show) = create_signal(false);

    // 当前的颜色模式
    let (mode, set_mode) = create_signal(0);

    view! {
        <div class=clsx!(class) on:click=move |_| { set_show(!show()) }>
            test
        </div>
    }
}
