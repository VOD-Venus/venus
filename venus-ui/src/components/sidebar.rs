use leptos::*;
use leptos_router::use_location;

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

    let children = move |n: Navi| {
        view! {
            <li>
                <a
                    href=n.path
                    class="mb-2"
                    class=("btn-active", move || n.path == location.pathname.get())
                >
                    {n.name}
                </a>
            </li>
        }
    };

    view! {
        <nav class="sm:flex flex-col w-56 max-w-xs px-5 py-6 bg-gray-100 dark:bg-rua-gray-900 hidden">
            // logo
            <div class="flex justify-center w-full">
                <img alt="venus" src="public/venus.svg" class="object-contain w-28 h-28" />
            </div>

            // nav
            <div class="flex flex-col justify-between h-full felx-1">
                <div class="flex flex-col my-4">
                    <ul class="my-4 menu bg-base-200 rounded-box">
                        <For each=move || NAVI key=|n| n.path children=children />
                    </ul>
                </div>
            </div>
        </nav>
    }
}
