use leptos::*;
use leptos_router::use_location;

use crate::clsx;

struct Navi {
    pub name: &'static str,
    pub path: &'static str,
}
const NAVI: [Navi; 5] = [
    Navi {
        name: "Proxies",
        path: "/",
    },
    Navi {
        name: "Settings",
        path: "/settings",
    },
    Navi {
        name: "Logging",
        path: "/logging",
    },
    Navi {
        name: "Editor",
        path: "/editor",
    },
    Navi {
        name: "About",
        path: "/about",
    },
];

#[component]
pub fn Sidebar() -> impl IntoView {
    let location = use_location();

    view! {
        <nav class=clsx!(
            "w-56 max-w-xs flex",
            "py-6 px-5",
            "bg-gray-100 flex-col",
            "dark:bg-rua-gray-900"
        )>
            // logo
            <div class=clsx!("flex w-full justify-center")>
                <img alt="venus" src="public/venus.svg" class=clsx!("object-contain w-28 h-28") />
            </div>

            // nav
            {location.pathname}
            <div class=clsx!("flex flex-col justify-between", "felx-1 h-full")>
                <div class=clsx!("my-4")>
                    <For
                        each=move || NAVI
                        key=|n| n.path
                        children=move |n| {
                            view! {
                                <a href=n.path>
                                    <button class=clsx!(
                                        "btn",
                                        if n.path == location.pathname.get() {"btn-active"} else {""}
                                    )>{n.name}</button>
                                </a>
                            }
                        }
                    />
                </div>
            </div>
        </nav>
    }
}
