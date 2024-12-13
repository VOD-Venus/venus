use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::hooks::use_global_ui;

struct Navi {
    pub name: &'static str,
    pub path: &'static str,
    pub icon: &'static str,
}
const NAVI: [Navi; 5] = [
    Navi {
        name: "Proxies",
        path: "/home",
        icon: "icon-[solar--server-2-bold-duotone]",
    },
    Navi {
        name: "Settings",
        path: "/settings",
        icon: "icon-[solar--settings-minimalistic-bold-duotone]",
    },
    Navi {
        name: "Logging",
        path: "/logging",
        icon: "icon-[solar--file-text-bold-duotone]",
    },
    Navi {
        name: "Editor",
        path: "/editor",
        icon: "icon-[solar--code-file-bold-duotone]",
    },
    Navi {
        name: "About",
        path: "/about",
        icon: "icon-[solar--planet-bold-duotone]",
    },
];

#[component]
fn SidebarContent() -> impl IntoView {
    let location = use_location();
    let ui = use_global_ui();

    let children = move |n: Navi| {
        let ui = use_global_ui();

        view! {
            <li>
                <a
                    href=n.path
                    class="mb-2 flex items-center transition-all duration-300"
                    class=("btn-active", move || n.path == location.pathname.get())
                >
                    <span class=format!("{}", n.icon)></span>
                    <span class=("hidden", move || !ui.sidebar_open.get())>{n.name}</span>
                </a>
            </li>
        }
    };

    view! {
        <>
            // logo
            <div class="flex justify-center w-full">
                <img
                    alt="venus"
                    src="public/venus.svg"
                    class="object-contain max-w-[7rem] max-h-28"
                />
            </div>

            // nav
            <div class="flex flex-col justify-between h-full felx-1">
                <div class="flex flex-col my-4">
                    <ul class="my-4 menu bg-transparent rounded-box">
                        <For each=move || NAVI key=|n| n.path children=children />
                    </ul>
                </div>

                <div>
                    <button
                        class="btn btn-ghost"
                        on:click=move |_| ui.sidebar_open.set(!ui.sidebar_open.get())
                    >
                        <span class="icon-[solar--square-double-alt-arrow-left-bold-duotone]"></span>
                    </button>
                </div>
            </div>
        </>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    let ui = use_global_ui();

    view! {
        <>
            <nav
                class="sm:flex flex-col max-w-xs px-5 py-6 bg-gray-100 dark:bg-rua-gray-900 hidden transition-all duration-300"
                class=("w-56", move || ui.sidebar_open.get())
            >
                <SidebarContent />
            </nav>
        </>
    }
}

#[component]
pub fn SidebarMobile() -> impl IntoView {
    view! {
        <>
            <div class="navbar bg-transparent p-0 mt-[-2rem] mx-[-1rem] sm:hidden">
                <div class="flex-none">
                    <div class="z-10 sm:hidden relative">
                        <input id="my-drawer" type="checkbox" class="drawer-toggle" />
                        <label for="my-drawer" class="btn drawer-button btn-square btn-ghost">
                            <span
                                class="icon-[solar--hamburger-menu-line-duotone]"
                                style="width: 24px; height: 24px;"
                            ></span>
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
                    <a class="btn btn-ghost text-xl p-0">Venus</a>
                </div>
            </div>
        </>
    }
}
