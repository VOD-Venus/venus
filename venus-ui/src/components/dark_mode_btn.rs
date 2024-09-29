use std::borrow::Cow;

use leptos::*;

use crate::clsx;

/// 颜色模式切换下拉框
#[component]
pub fn DarkMode(#[prop(optional)] class: Cow<'static, str>) -> impl IntoView {
    let themes = [
        "light",
        "dark",
        "cupcake",
        "bumblebee",
        "emerald",
        "corporate",
        "synthwave",
        "retro",
        "cyberpunk",
        "valentine",
        "halloween",
        "garden",
        "forest",
        "aqua",
        "lofi",
        "pastel",
        "fantasy",
        "wireframe",
        "black",
        "luxury",
        "dracula",
        "cmyk",
        "autumn",
        "business",
        "acid",
        "lemonade",
        "night",
        "coffee",
        "winter",
        "dim",
        "nord",
        "sunset",
    ];
    let (current_theme, set_current_theme) = use_context::<(ReadSignal<&str>, WriteSignal<&str>)>()
        .expect("to have found the setter provided");
    create_effect(move |_| {
        logging::log!("{}", current_theme());
    });

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
                    each=move || themes
                    key=|theme| theme.to_string()
                    children=move |theme| {
                        view! {
                            <li class=clsx!("w-full") on:click=move |_| set_current_theme(theme)>
                                <a class:active=move || { current_theme() == theme }>{theme}</a>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
