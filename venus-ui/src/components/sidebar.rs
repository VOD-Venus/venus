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
        <nav class="flex flex-col w-56 max-w-xs px-5 py-6 bg-gray-100 dark:bg-rua-gray-900">
            // logo
            <div class=clsx!("flex w-full justify-center")>
                <img alt="venus" src="public/venus.svg" class=clsx!("object-contain w-28 h-28") />
            </div>

            // nav
            <div class=clsx!("flex flex-col justify-between", "felx-1 h-full")>
                <div class=clsx!("my-4 flex flex-col")>
                    <ul class="my-4 menu bg-base-200 rounded-box">
                        <For
                            each=move || NAVI
                            key=|n| n.path
                            children=move |n| {
                                view! {
                                    <li>
                                        <a
                                            href=n.path
                                            class="mb-2"
                                            class=(
                                                "btn-active",
                                                move || n.path == location.pathname.get(),
                                            )
                                        >
                                            {n.name}
                                        </a>
                                    </li>
                                }
                            }
                        />
                    </ul>
                </div>
            </div>
        </nav>
    }
}
