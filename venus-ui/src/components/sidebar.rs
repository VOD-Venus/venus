use leptos::*;
use leptos_router::use_location;

struct Navi {
    pub name: &'static str,
    pub path: &'static str,
}
const NAVI: [Navi; 5] = [
    Navi {
        name: "Proxies",
        path: "/home",
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
fn SidebarContent() -> impl IntoView {
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
        <>
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
        </>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <>
            <nav class="sm:flex flex-col w-56 max-w-xs px-5 py-6 bg-gray-100 dark:bg-rua-gray-900 hidden">
                <SidebarContent />
            </nav>
        </>
    }
}

#[component]
pub fn SidebarMobile() -> impl IntoView {
    view! {
        <>
            <div class="navbar bg-base-100 p-0 mt-[-2rem] mx-[-1rem] sm:hidden">
                <div class="flex-none">
                    <div class="z-10 sm:hidden relative">
                        <input id="my-drawer" type="checkbox" class="drawer-toggle" />
                        <label for="my-drawer" class="btn drawer-button btn-square btn-ghost">
                            <img src="public/images/components/hamburg.svg" alt="hamburger icon" />
                        </label>
                        <div class="drawer-side">
                            <label
                                for="my-drawer"
                                aria-label="close sidebar"
                                class="drawer-overlay"
                            ></label>
                            <ul class="menu bg-base-200 text-base-content min-h-full w-72 p-4">
                                <SidebarContent />
                            </ul>
                        </div>
                    </div>
                </div>

                <div class="flex-1">
                    <a class="btn btn-ghost text-xl">Venus</a>
                </div>
            </div>
        </>
    }
}
