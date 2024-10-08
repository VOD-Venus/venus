use std::borrow::Cow;

use leptos::*;
use leptos_use::ColorMode;

use crate::{clsx, consts::COLOR_MODE};

/// 颜色模式切换下拉框
#[component]
pub fn DarkMode(#[prop(optional)] class: Cow<'static, str>) -> impl IntoView {
    let (mode, set_mode) = use_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>()
        .expect("to have found the setter provided");

    view! {
        <div class=clsx!("dropdown", class)>
            <div tabindex="0" role="button" class="btn m-1">
                Themes
            </div>
            <ul
                tabindex="0"
                class=clsx!(
                    "dropdown-content menu bg-base-100",
                    "rounded-box z-[1] w-32 p-2 shadow",
                    "h-64 overflow-y-auto flex",
                    "flex-col flex-nowrap"
                )
            >
                <For
                    each=move || COLOR_MODE
                    key=|theme| theme.to_string()
                    children=move |theme| {
                        view! {
                            <li class=clsx!("w-full") on:click=move |_| set_mode.set(theme.into())>
                                <a class:active=move || {
                                    mode.get().to_string() == theme
                                }>{theme}</a>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}